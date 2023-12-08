//use crate::permission;
use crate::machine_hiding::os_detection;
use std::fs;
//use std::io::Result;
use std::fs::File;
use std::io::{self, Write};
use walkdir::WalkDir;
use std::path::{PathBuf, Path};
use crate::behaviour_hiding::output;
use crate::machine_hiding::file_system_operation::file_permission;
use std::fs::OpenOptions;
//use std::io::prelude::*;
// TODO: create a trait
pub struct FileStruct {
    pub file_name: String,
    perm:file_permission::Permission,
    cwd: String
}

impl FileStruct {
    pub fn new(file_name: String) -> FileStruct {
        let cwd = os_detection::pwd();
        
        let perm = file_permission::Permission{
            readable: true,
            writable: true,
        };
        //println!("{} {}",perm.writable,perm.readable);
        FileStruct { file_name, perm, cwd }
    }

    pub fn get_file_name(&self) -> &String{
        return &self.file_name;
    }

    pub fn create_file(&self) -> io::Result<()> {
        let mut filepath = PathBuf::from(&self.cwd);
        filepath.push(&self.file_name);
    
        match File::create(&filepath) {
            Ok(_) =>{} //output::print_message("File created successfully"),
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
    
    pub fn read(&self) ->String {
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();

        if self.perm.readable == true{
            return fs::read_to_string(fpr).unwrap_or_else(|err| {
                "Failed to Read, please check the file is exist or not".to_string()
            });
        }else{  
            return "The file cannot be read, you have to acquire permission first.".to_string();
        }
    }

    pub fn remove(&self)-> io::Result<()>{
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();

        match fs::remove_file(fpr) {
            Ok(_) =>println!("{} has been successfully removed!",self.file_name),
            Err(e) => {}
        }
        Ok(())    
    }

    pub fn mv(&self, target:&str) -> io::Result<()>{
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();
        
        let abs_target = self.cwd.clone()+"/"+target+"/"+self.file_name.clone().as_str(); 
        println!("{}",fpr);
        match fs::rename(fpr,abs_target){
            Ok(_)=> println!("{} has been successfully moved!", self.file_name),
            Err(e) => println!("Failed to move the file, please check the file name and the target path!")
        }
        Ok(())
    }
    pub fn file_is_exist(&self)->bool{
        let fpr = self.cwd.clone()+"/"+self.file_name.clone().as_str();
        let path = PathBuf::from(fpr);

        if path.exists() {
            return true;
        } else {
            return false;
        }
    }
}

// create_folder and create_file should be in one method of function
pub fn create_folder(create_dir: &str) -> std::io::Result<()> {
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(create_dir);

    match fs::create_dir_all(&path) {
        Ok(_) => {
            //println!("Folder created successfully.");
            Ok(())
        },
        Err(e) => {
            println!("Failed to create folder");
            //Err(e)
            Ok(())
        }
    }
}

pub fn remove_folder(remove_dir:&str)->std::io::Result<()>{
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(remove_dir);

    match fs::remove_dir_all(&path) {
        Ok(_) => {
            Ok(())
        },
        Err(e) => {
            println!("Failed to remove folder");
            //Err(e)
            Ok(())
        }
    }
}

pub fn folder_is_exist(folder_name:&str)->bool{
    let cwd = os_detection::pwd();
    let mut path = PathBuf::from(cwd);
    path.push(folder_name);
    path.exists() && path.is_dir()
}

pub fn get_file_list()->Vec<String>{
    let cwd: String = os_detection::pwd();
    let file_paths: Vec<String> = WalkDir::new(cwd)
    .into_iter()
    .filter_map(|e| e.ok())
    .filter(|e| e.path().is_file())
    .map(|e| e.path().to_string_lossy().into_owned())
    .collect();

    // for path in file_paths {
    //     println!("{}", path);
    // }
    return file_paths;
}

pub fn clone(target_dir:&str)-> std::io::Result<()>{
    let cwd: String = os_detection::pwd();
    let cwd_path = Path::new(cwd.as_str());
    let td = Path::new(target_dir);
    let td_shield = Path::new(target_dir).join(".shield");//full path of a .shield folder in target path
    if td_shield.exists() && td_shield.is_dir() {
        match fs::read_dir(cwd_path) {
        Ok(mut entries) => {
            if entries.next().is_none() {
                for entry in WalkDir::new(target_dir) {
                    let entry = entry?;
                    let path = entry.path();
                    let relative_path = path.strip_prefix(target_dir).unwrap();
                    let target_path = cwd_path.join(relative_path);
            
                    if path.is_dir() {
                        fs::create_dir_all(&target_path)?;
                    } else {
                        fs::copy(path, &target_path)?;
                    }
                }              
            } else {
                println!("The directory is not empty. Failed to clone!");
            }
        }
        Err(e) => {
            println!("Failed to read the directory: {}", e);
        }
    }
    } else {
        println!("This is not a shield repository, please check the target folder is exist and a valid shield repository or not!");
    }
    Ok(())
}