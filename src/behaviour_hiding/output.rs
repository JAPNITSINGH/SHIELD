pub mod behaviour_hiding{
    pub mod output_management{
        pub fn print_message(message:&str){
            println!("{}",message);
        }
        
        pub fn print_help(){
            println!("These are shield commands commonly used:

- Initialize a new repository:   shield init
- Clone an existing repository:  shield clone <\"repo_url\">
- Check your current status:    shield status
- Add files to staging:         shield add <\"file_name \">
- Commit your changes:          shield commit <\"your message\">
- Push changes to remote:       shield push
- Pull updates from remote:     shield pull
- Remove a file from repository:  shield remove <\"file_name\">
- Move a file to a different directory: shield mv <\"file_name\"> <\"new_path\">
            ");
        }
        
        pub fn print_welcome(){
            print!("Welcome to S.H.I.E.L.D - The Distributed Version Control System

S.H.I.E.L.D efficiently handles projects of any size with speed and accuracy.
It allows you to collaborate seamlessly with your team, no matter where you are.

Getting started:
- Initialize a new repository:   shield init
- Clone an existing repository:  shield clone <\"repo_url\">
- Check your current status:    shield status
- Add files to staging:         shield add <\"file_name\">
- Commit your changes:          shield commit <\"your message\">
- Push changes to remote:       shield push
- Pull updates from remote:     shield pull

For more commands and usage information, type 'shield help'.

Thank you for choosing S.H.I.E.L.D for your version control needs.
Happy coding!

(c) 2023 AVENGERS Team - CSC 253/453
            ");
        }
    }
}