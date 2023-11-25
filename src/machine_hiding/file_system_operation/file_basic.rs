//use crate::permission;
use crate::machine_hiding::os_detection;
use std::fs;
//use std::io::Result;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use crate::behaviour_hiding::output;
use crate::machine_hiding::file_system_operation::file_permission;
use std::fs::OpenOptions;
use std::io::prelude::*;

pub struct FileStruct {
    file_name: String,
    perm:file_permission::Permission,
    cwd: String
}

impl FileStruct {
    pub fn new(file_name: String) -> FileStruct {
        let cwd = os_detection::pwd();
        let fp = file_name.clone()+&cwd;
        let mut perm = file_permission::Permission{
            readable: true,
            writable: true,
        };
        println!("{} {}",perm.writable,perm.readable);
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
    pub fn write(&self, content:&str) -> std::io::Result<()>  {
        if self.perm.writable == true{
            let mut w = OpenOptions::new().append(true).open(&self.file_name)?;
            let c = content.to_string() + "\n";
            w.write_all(c.as_str().as_bytes())?;
        }else{
            output::print_message("The file cannot be written, you have to acquire permission first.");
        }
        Ok(())
    }
    
    pub fn read(&self) {
        if self.perm.readable == true{

        }else{
            output::print_message("The file cannot be read, you have to acquire permission first.");
        }
    }
    
}


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