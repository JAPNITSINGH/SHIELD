use std::fs::{self, File};
use std::ops::IndexMut;
use std::path::Path;
use std::io::{self, Write, Read};
use crate::machine_hiding::file_system_operation::file_basic::FileStruct;
use crate::machine_hiding::{os_detection,file_system_operation::file_basic, file_log};

struct Commit{
    commit_hash: String
}

struct RootNode {
    root_node_id: String
}

impl Commit {
    pub fn new() -> Self{
        let name = "COMMIT".to_string();
        Commit {
            commit_hash: file_log::generate_hash_id(&name) 
        }
    }

    pub fn get_commit_id(&self) -> &String {
       return  &self.commit_hash;
    }
}

impl RootNode {
    pub fn new() -> Self{
        let name: String = "ROOTNODE".to_string();
        RootNode { 
            root_node_id: file_log::generate_hash_id(&name) 
        }
    }

    pub fn get_root_id(&self) -> &String{
        return &self.root_node_id;
    }
}


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

pub fn commit_files(){
    let new_commit: Commit = Commit::new();
    let root_node_of_tree: RootNode = RootNode::new();
    
    if(is_first_commit()) {
        let mut f_master = file_basic::FileStruct::new(".shield/refs/heads/master".to_string());
        let mut f_master_logs = file_basic::FileStruct::new(".shield/logs/refs/heads/master".to_string());
        let mut f_commit_file = file_basic::FileStruct::new(".shield/objects/".to_string() + new_commit.get_commit_id());
        let mut f_root_file = file_basic::FileStruct::new(".shield/objects/".to_string() + root_node_of_tree.get_root_id());
        //println!("{}", f_root_file.file_name);
        let mut f_index = file_basic::FileStruct::new(".shield/index".to_string());
       // println!("{}",&f_index.file_name);
        let index_file_content = f_index.read();
        let master_log_content = "0000000000000000000000000000000000000000 ".to_string() + new_commit.get_commit_id();

        f_master.create_file();
        f_master_logs.create_file();
        f_commit_file.create_file();
        f_root_file.create_file();

        f_master_logs.write(&master_log_content[..]);
        f_master.write(new_commit.get_commit_id());
        f_commit_file.write(&root_node_of_tree.get_root_id());
        f_root_file.write(&index_file_content[..]);

        println!("{}", &index_file_content);

        f_index.remove();
    }
    else {

    }

}

pub fn add_files(){
    let pwd = os_detection::pwd();
    let is_repo = file_basic::folder_is_exist(".shield");
    let files_list: Vec<FileStruct> = file_basic::get_file_list();


    if is_repo {
        //  if (is_first_commit()) { 
        //     add_file_hash(List<File>)
        // }
        // else{
        //     compare_all_files_with_last_commit()
        //     add_file_hash(List<Files>);
        // }

        // THIS STATEMENT SHOULD BE OUTSIDE ITERATOR
        let index_file = FileStruct::new(".shield/index".to_string());
        index_file.create_file();

        // TODO: PUT THESE IN AN ITERATOR OVER files_list
        files_list.iter().for_each(|file| {
            let content = file.read();
            //println!("{}", &content);
            let hash = file_log::generate_hash_id(file.get_file_name());
            //println!("{}", file.get_file_name());
            let new_file_name = ".shield/objects/".to_string() + &hash;
            let f = file_basic::FileStruct::new(new_file_name);
        
            if is_first_commit() {
                f.create_file();
                f.write(&content);
                index_file.write(&hash);
            }
            else{
                // TODO: Must be implemented after commit
                index_file.write(&hash);
            }
        });
        // ITERATOR ENDS HERE
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

fn is_first_commit() -> bool{
    let master_file: FileStruct = FileStruct::new(".shield/refs/heads/master".to_string());
    return !master_file.file_is_exist();
}
