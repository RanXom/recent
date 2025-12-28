use std::{fs, io};
use std::path::Path;
use std::time::SystemTime;

fn main() -> io::Result<()> {
    let mut folder_path = String::new();

    println!("Folder Path: ");
    io::stdin()
        .read_line(&mut folder_path)
        .expect("Failed to read line");

    let path = Path::new(folder_path.trim());
    let mut entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort_by_key(|path| {
        fs::metadata(path)
            .and_then(|m| m.modified())
            .unwrap_or(SystemTime::UNIX_EPOCH)
    });
    entries.reverse();

    print!("{:?}", entries);
    Ok(())
}
