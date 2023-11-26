use std::fs;
use std::path::Path;
use std::io;

use crate::machine_hiding::{os_detection,file_system_operation::file_basic};

fn init_shield()-> io::Result<()>{

    // Create the main repository directory

    let _ = file_basic::create_folder(".shield");

    // Create subdirectories for DVCS structure
    file_basic::create_folder(".shield/branches");
    file_basic::create_folder(".shield/tags");
    file_basic::create_folder(".shield/objects");
    file_basic::create_folder(".shield/refs");
    file_basic::create_folder(".shield/logs");
    file_basic::create_folder(".shield/info");

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
