
use sha1::{Sha1, Digest};
use std::time::{SystemTime, UNIX_EPOCH};


use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, ErrorKind};


pub fn generate_hash_id(filename: &String) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.as_secs();

    let mut hasher = Sha1::new();

    hasher.update(time.to_be_bytes());
    hasher.update(filename);

    let hash = hasher.finalize();
    return format!("{:x}", hash)
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
    fs::read_dir(from_dir)?
        .filter_map(Result::ok)  
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .try_for_each(|path| {
            let dest_path = to_dir.join(path.file_name().ok_or_else(|| io::Error::new(ErrorKind::Other, "Missing file name"))?);
            fs::copy(&path, &dest_path)?;
            Ok(())
        })
}






// #[derive(Debug, Clone)]
// pub struct Log {
//     file_path: String,
//     timestamp: u64,
//     version_id: String,
//     modification_content: String,
// }

// impl Log {
//     pub fn new(file_path: String, modification_content: String) -> Log {
//         let timestamp = SystemTime::now()
//             .duration_since(UNIX_EPOCH)
//             .expect("Time went backwards")
//             .as_secs();

//         let version_id = format!("{}-{}", file_path, timestamp); 

//         Log {
//             file_path,
//             timestamp,
//             version_id,
//             modification_content,
//         }
//     }
// }


// pub fn cat() {}

// pub fn retrieve() {
//     // This function gets contents of a file at a partivular version.
// }
