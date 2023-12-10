use crate::{machine_hiding::file_system_operation::file_basic::{self, FileStruct}, repository_hiding::repository_local::repository_versioning};


pub fn merge(args:Vec<&str>) {
    println!("Entering Merge");
    //check for right args
    //fetch head commit from current branch
    //fetch head commit from the argument branch
    //fetch list of file struct from current branch
    //fetch list of file struct from the argument branch
    
    //for each file in currentbranch do1
        // if filename is present in argument_file_struct
            // merge two files content
        // else
            // skip merging the file
    // end do1

    //for each file in argument branch
        // if file is not there in current file struct
            // add this file
    
    // maybe the user needs to do it by himself
    // shield add 
    // shield commit
    if args.len()!=3 {
        println!("Invalid checkout command syntax");
        return;
    }

    let mut current_branch = FileStruct::new("shield/HEAD".to_string());
    let mut merge_branch_name = args[2];

    if (!repository_versioning::branch_exists(merge_branch_name)) {
        return;
    }

    let mut f1 = FileStruct::new(".\\branch_1_file.txt".to_string());
    let mut f2 = FileStruct::new(".\\branch_2_file.txt".to_string());
    match merge_files(&f1, &f2) {
        Ok(merged_content) => {
            println!("Delete and Writing down");
           f1.remove();

            let mut f1_new = FileStruct::new("./branch_1_file.txt".to_string());
            f1_new.create_file();
            f1_new.write(&merged_content[..]);
        }
        Err(err) => {}
    }
}

fn merge_files(f1: &FileStruct, f2: &FileStruct) -> Result<String, std::io::Error> {
    println!("Entering Merge Files");
    let base_content = f1.read();
    let other_content = f2.read();
    let base_lines: Vec<&str> = base_content.lines().collect();
    let other_lines: Vec<&str> = other_content.lines().collect();


    println!("{}",base_content);
   // println!("{}",other_content);
    let mut merged_content = String::new();

    let mut i = 0;
    let mut j = 0;

    while i < base_lines.len() || j < other_lines.len() {
        if i < base_lines.len() && j < other_lines.len() {
            if base_lines[i] == other_lines[j] {
                merged_content.push_str(base_lines[i]);
                merged_content.push('\n');
                i += 1;
                j += 1;
            } else {
                // Conflict occurred, handle it as needed
                merged_content.push_str("<<<<<<< Base\n");
                while i < base_lines.len() && base_lines[i] != other_lines[j] {
                    merged_content.push_str(base_lines[i]);
                    merged_content.push('\n');
                    i += 1;
                }
                merged_content.push_str("=======\n");
                while j < other_lines.len() && base_lines[i - 1] != other_lines[j] {
                    merged_content.push_str(other_lines[j]);
                    merged_content.push('\n');
                    j += 1;
                }
                merged_content.push_str(">>>>>>> Other\n");
            }
        } else if i < base_lines.len() {
            // Remaining lines in the base file
            while i < base_lines.len() {
                merged_content.push_str(base_lines[i]);
                merged_content.push('\n');
                i += 1;
            }
        } else if j < other_lines.len() {
            // Remaining lines in the other file
            while j < other_lines.len() {
                merged_content.push_str(other_lines[j]);
                merged_content.push('\n');
                j += 1;
            }
        }
    }

    // let changeset = Changeset::new(&base_content, &other_content, "");

    // let merged_content = match changeset {
    //     [Difference::Same(content)] => content.to_string(),
    //     [_, Difference::Same(content), _] => content.to_string(),
    //     [_, Difference::Same(content)] => content.to_string(),
    //     _ => {
    //         eprintln!("Conflict occurred while merging");
    //         base_content // For simplicity returning base_content
    //     }
    // };

    println!("{}", merged_content);
    Ok(merged_content)
}
