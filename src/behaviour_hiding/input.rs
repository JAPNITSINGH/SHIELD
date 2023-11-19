use crate::behaviour_hiding::output;
use crate::machine_hiding::os_detection;
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

        // if user_input == "quit" {
        //     break;
        // }

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
                        _ => println!("{} is not a valid shield command, please type shield help if you have any questions",args[1])
                    }
        }
    }
}
