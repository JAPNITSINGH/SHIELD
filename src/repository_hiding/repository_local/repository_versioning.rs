use std::fs::{self, File};
use std::path::Path;
use std::io::{self, Write, Read};
use crate::machine_hiding::file_system_operation::file_basic::FileStruct;
use crate::machine_hiding::{os_detection,file_system_operation::file_basic, file_log};

fn read_current_head(repo_path: &str) -> io::Result<String> {
    let head_path = Path::new(repo_path).join(".shield/HEAD");
    let mut head_file = fs::File::open(head_path)?;
    let mut head_commit_id = String::new();
    head_file.read_to_string(&mut head_commit_id)?;
    Ok(head_commit_id.trim().to_string())
}

fn branch( repo_path: &str,branch_name: &str, user_id: &str) -> io::Result<()> {
    let head_commit_id = read_current_head(repo_path)?;

    let _ = file_basic::create_folder(".shield");
    // Create new files for the branch
    let logs_path = Path::new(repo_path).join(format!(".shield/logs/refs/HEAD/{}", branch_name));
    let refs_path = Path::new(repo_path).join(format!(".shield/refs/HEAD/{}", branch_name));

    // Write to LOGS
    let mut logs_file = fs::File::create(logs_path)?;
    writeln!(logs_file, "0 {}\n{} {}", head_commit_id, head_commit_id, user_id)?;

    // Write to REFS
    let mut refs_file = fs::File::create(refs_path)?;
    writeln!(refs_file, "{}", head_commit_id)?;

    Ok(())
}

pub fn branch_main(args:Vec<&str>) {
    if args.len()<=2{
        println!("please enter a branch name");
    }else if args.len()>3{
        println!("No space in a branch name, or you can add double quotes on the branch name");
    }else{   
        let branch_name = args[2];
        let user_id = "user123";
        let repo_path=os_detection::pwd();
        let is_repo = file_basic::folder_is_exist(".shield");

        if(is_repo) {

            match branch( &repo_path,branch_name, user_id) {
                Ok(_) => println!("Branch '{}' created.", branch_name),
                Err(e) => println!("Failed to create branch: {}", e),
            }

        }
        else {
            println!("Not a shield repository"); 
        }
    }
}

pub fn add_files(){
    let pwd = os_detection::pwd();
    let is_repo = file_basic::folder_is_exist(".shield");
    let files_list: Vec<FileStruct> = get_files_list(&pwd);



    if (is_repo) {
        if (is_fist_commit()) {
            let hash = file_log::generate_hash_id(files_list[0].get_file_name());
            let new_file_name = ".shield/objects/".to_string() + &hash;
            let mut f = file_basic::FileStruct::new(new_file_name);
            let _ = f.create_file();
        }
        //     add_file_hash(List<File>)
        // }
        // else{
        //     compare_all_files_with_last_commit()
        //     add_file_hash(List<Files>);
        // }
    }
    else{
        println!("Not a shield repository");
    }
}

fn add_file_hash(){
    // for each item in list
    // let hash_id = machine_hash
    // machine.createfile(hash_id)
    // contents = read_from_durrent_item
    // manchine.add_contents_to_file(hash_id, contents)
}

fn is_fist_commit() -> bool{
    true
}

fn get_files_list(pwd: &String) -> Vec<FileStruct> {
    let dummy_list: Vec<FileStruct> = vec![FileStruct::new("MyFIle.txt".to_string())];
    return dummy_list;
}