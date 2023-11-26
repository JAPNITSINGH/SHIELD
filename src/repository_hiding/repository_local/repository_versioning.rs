use std::fs;
use std::path::Path;
use std::io::{self, Write, Read};
use crate::machine_hiding::{os_detection,file_system_operation::file_basic};

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
        println!("please enter a file name");
    }else if args.len()>3{
        println!("No space in a file name, or you can add double quotes on the file name");
    }else{   
        let branch_name = args[2];
        let user_id = "user123";
        let repo_path=os_detection::pwd();

        match branch( &repo_path,branch_name, user_id) {
            Ok(_) => println!("Branch '{}' created.", branch_name),
            Err(e) => println!("Failed to create branch: {}", e),
        }
    }
}
