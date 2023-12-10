use std::fs;
use std::path::Path;
use std::io;

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
    let _ = f.write("refs/heads/master");



    file_basic::create_folder(".shield/logs/refs");
    file_basic::create_folder(".shield/logs/refs/heads");
    let mut t = file_basic::FileStruct::new(".shield/logs/HEAD".to_string());
    let __ = t.create_file();

    file_basic::create_folder(".shield/refs/heads");
    file_basic::create_folder(".shield/refs/remote");
    file_basic::create_folder(".shield/refs/tags");

    Ok(())
}

pub fn init_main() {
    match init_shield() {
        Ok(_) => println!("Repository initialized"),
        Err(e) => println!("Failed to initialize repository: {}", e),
    }
}
