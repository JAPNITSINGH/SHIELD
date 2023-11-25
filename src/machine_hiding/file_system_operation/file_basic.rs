//use crate::permission;
use crate::machine_hiding::os_detection;
use std::fs;
//use std::io::Result;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::behaviour_hiding::output;
pub struct FileStruct {
    file_name: String,
    perm: bool,
    cwd: String
}

impl FileStruct {

    pub fn new(file_name: String) -> FileStruct {
        let perm = true;
        let cwd = os_detection::pwd();
        FileStruct { file_name, perm, cwd }
    }

    pub fn create_file(&self) -> io::Result<()> {
        let mut filepath = PathBuf::from(&self.cwd);
        filepath.push(&self.file_name);
    
        match File::create(&filepath) {
            Ok(_) => output::print_message("File created successfully"),
            Err(e) => {
                output::print_message("Failed to create file");
                return Err(e);
            }
        }
    
        Ok(())
    }
}
// pub fn add(f:File){
//     todo!()
// }

// pub fn remove(f:File){
//     todo!()
// }

// pub fn read(f:File){
//     todo!()
// }

// pub fn write(f:File){
//     todo!()
// }

// pub fn mv(f:File, addr:&str){
//     todo!()
// }

// pub fn rename(f:File){
//     todo!()
// }

pub fn create_folder(create_dir: &str) -> std::io::Result<()> {
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(create_dir);

    match fs::create_dir_all(&path) {
        Ok(_) => {
            println!("Folder created successfully.");
            Ok(())
        },
        Err(e) => {
            println!("Failed to create folder: {:?}", e);
            //Err(e)
            Ok(())
        }
    }
}

pub fn write() {
    
}

pub fn read() {
    
}
