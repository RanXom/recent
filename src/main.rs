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

fn visit_dirs(path: &PathBuf) -> io::Result<Vec<PathBuf>> {
    let mut entries: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let metadata = fs::metadata(&entry_path)?;

        if metadata.is_file() {
            entries.push(entry_path);
        } else if metadata.is_dir() {
            let mut sub_entries = visit_dirs(&entry_path)?;
            entries.append(&mut sub_entries);
        }
    }

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
    let dir_path = PathBuf::from(input_dir());
    let dirs = visit_dirs(&dir_path)?;
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
