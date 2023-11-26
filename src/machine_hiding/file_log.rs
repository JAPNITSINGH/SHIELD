use std::time::{SystemTime, UNIX_EPOCH};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub fn generate_commit_id(file_name: &str) -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    let time = since_the_epoch.as_secs() * 1_000 + since_the_epoch.subsec_millis() as u64;

    let mut hasher = DefaultHasher::new();
    file_name.hash(&mut hasher);
    time.hash(&mut hasher);
    hasher.finish()
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
