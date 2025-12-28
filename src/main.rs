use std::{fs, io};
use std::path::PathBuf;
use std::time::SystemTime;

fn input_dir() -> String {
    let mut dir = String::new();

    println!("Folder Path: ");
    io::stdin()
        .read_line(&mut dir)
        .expect("Failed to read line");

    dir.trim().to_string()
}

fn visit_dirs(path: String) -> io::Result<Vec<PathBuf>> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    Ok(entries)
}

fn get_modified_times(path: Vec<PathBuf>) -> io::Result<Vec<(PathBuf, SystemTime)>> {
    Ok(
        path
            .into_iter()
            .filter_map(|path| {
                let metadata = fs::metadata(&path).ok()?;

                if metadata.is_file() {
                    let last_modified = metadata.modified().ok()?;
                    Some((path, last_modified))
                } else {
                    None
                }
            })
            .collect()
    )
}

fn main() -> io::Result<()> {
    let dirs = visit_dirs(input_dir())?;
    let last_modified = get_modified_times(dirs)?;

    for (path, time) in last_modified {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>");

        println!("{file_name} - {:?}", time);
    }

    Ok(())
}
