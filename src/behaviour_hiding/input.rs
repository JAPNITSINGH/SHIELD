pub mod behaviour_hiding{
    pub mod input_management{
        use std::io::{self, Write};
        use crate::behaviour_hiding::output::behaviour_hiding::output_management;
        use shellwords;

        pub fn initialization(){
            output_management::print_welcome();
            loop {
                let mut user_input = String::new();
        
                print!("dvcs>");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut user_input).unwrap();
        
                let user_input = user_input.trim_end();
        
                if user_input == "quit" {
                    break;
                }
        
                // divide the input message to a vec, each element represents a word, for exampele, user input = shield add, args = ["shield", "add"].

                let args = match shellwords::split(user_input){
                    Ok(args) => args,
                    Err(err)=>{
                        eprintln!("Error:{}",err);
                        return;
                    },
                };
                let args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
                println!("{:?}",args);
                divide_command(args);
            }    
        }   
        
        fn divide_command(v:Vec<&str>){
            if v.is_empty(){
            }
        }
        
        
        
        
    }
}