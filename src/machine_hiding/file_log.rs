
use sha1::{Sha1, Digest};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn generate_commit_id(filename: String) -> String {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.as_secs();

    let mut hasher = Sha1::new();

    hasher.update(time.to_be_bytes());
    hasher.update(filename);

    let hash = hasher.finalize();
    format!("{:x}", hash)
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
