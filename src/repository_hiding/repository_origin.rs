use std::fs;
use std::io;

use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};
use walkdir::WalkDir;

use crate::machine_hiding::{os_detection,file_system_operation::file_basic};

fn init_shield()-> io::Result<()>{

    // Create the main repository directory

    let _ = file_basic::create_folder(".shield");

    // Create subdirectories for DVCS structure
    file_basic::create_folder(".shield/objects");
    file_basic::create_folder(".shield/refs");
    file_basic::create_folder(".shield/logs");
    // let mut f = file_basic::FileStruct::new(".shield/index".to_string());
    // f.create_file();
    
    let mut f = file_basic::FileStruct::new(".shield/HEAD".to_string());
    f.create_file();
    let _ = f.write("ref: refs/heads/master\n");



    file_basic::create_folder(".shield/logs/refs");
    file_basic::create_folder(".shield/logs/refs/heads");
    let mut t = file_basic::FileStruct::new(".shield/logs/HEAD".to_string());
    let __ = t.create_file();

    file_basic::create_folder(".shield/refs/heads");
    file_basic::create_folder(".shield/refs/remote");
    file_basic::create_folder(".shield/refs/tags");

    // Create a placeholder for the HEAD
    //fs::write(repo.join("HEAD"), "ref: refs/heads/master\n")?;

    Ok(())
}

pub fn init_main() {
    match init_shield() {
        Ok(_) => println!("Repository initialized"),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }
}

pub fn pull(from: &str, to: &str) -> io::Result<()> {
    let from_dir=Path::new(from);
    let to_dir=Path::new(to);

    if !from_dir.is_dir() {
        return Err(io::Error::new(ErrorKind::NotFound, "Source directory does not exist"));
    }
    if !to_dir.is_dir() {
        fs::create_dir_all(to_dir)?;
    }
    WalkDir::new(from_dir)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.into_path())
        .try_for_each(|path| {
            let relative_path = path.strip_prefix(from_dir).unwrap_or(&path);
            let dest_path = to_dir.join(relative_path);
            if path.is_dir() {
                fs::create_dir_all(&dest_path)
            } else {
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&path, &dest_path)?;
                Ok(())
            }
        })
}

pub fn push(from: &str, to: &str) -> io::Result<()> {
    let from_dir=Path::new(from);
    let to_dir=Path::new(to);

    if !from_dir.is_dir() {
        return Err(io::Error::new(ErrorKind::NotFound, "Source directory does not exist"));
    }
    if !to_dir.is_dir() {
        fs::create_dir_all(to_dir)?;
    }
    WalkDir::new(from_dir)
        .into_iter()
        .filter_map(Result::ok)
        .map(|entry| entry.into_path())
        .try_for_each(|path| {
            let relative_path = path.strip_prefix(from_dir).unwrap_or(&path);
            let dest_path = to_dir.join(relative_path);
            if path.is_dir() {
                fs::create_dir_all(&dest_path)
            } else {
                if let Some(parent) = dest_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&path, &dest_path)?;
                Ok(())
            }
        })
}
