use crate::behaviour_hiding::output;
use crate::machine_hiding::{os_detection,file_system_operation::file_basic};

use shellwords;
use std::{
    env::args,
    io::{self, Write},
};

pub fn initialization() {
    output::print_welcome();
    loop {
        let mut user_input = String::new();

        print!("dvcs>");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut user_input).unwrap();

        let user_input = user_input.trim_end();
        let user_input = user_input.trim_start();

        if user_input == "exit" {
            break;
        }

        // divide the input message to a vec, each element represents a word, for exampele, user input = shield add, args = ["shield", "add"].
        if !user_input.contains(' ') {
            let mut args = Vec::new();
            args.push(user_input);
            println!("{:?}", args);
            divide_command(args);
        } else {
            let args = match shellwords::split(user_input) {
                Ok(args) => args,
                Err(err) => {
                    eprintln!("Error:{}", err);
                    return;
                }
            };
            let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            println!("{:?}", args);
            divide_command(args);
        }
    }
}

fn divide_command(args: Vec<&str>) {
    if !args.is_empty() {
        if args[0] != "shield" {
            println!("{} is not a valid shield command, please type shield help if you have any questions",args[0]);
        } else if args.len() == 1 {
            println!("please tell us what's your command, if you don't know, type \"shield help\" to get some help");
        } else {
            match args[1]{
                        "help" =>  output::print_help(),
                        "quit" =>  std::process::exit(0),
                        "init" => output::print_help(),
                        "pwd" =>  println!("{}",os_detection::pwd()),
                        "createfile"=> process_create(args),
                        "createfolder" => process_create_folder(args),
                        _ => println!("{} is not a valid shield command, please type shield help if you have any questions",args[1])
                    }
        }
    }
}

fn process_create(args:Vec<&str>){
    if args.len()<=2{
        println!("please enter a file name");
    }else if args.len()>3{
        println!("No space in a file name, or you can add double quotes on the file name");
    }else{    
        let mut f = file_basic::FileStruct::new(args[2].to_string());
        f.create_file();
    }
}

fn process_create_folder(args:Vec<&str>){
    if args.len()<=2{
        println!("please enter a folder name");
    }else if args.len()>3{
        println!("No space in a folder name, use slash \"/\" to separate the folder name");
    }else{    
        //let mut f = file_basic::FileStruct::new();
        file_basic::create_folder(args[2]);
    }
}