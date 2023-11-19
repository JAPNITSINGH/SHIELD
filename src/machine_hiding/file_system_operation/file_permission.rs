pub mod Machine_Hiding{
    pub mod File_System_Operation{
        pub mod  File_Permission{
            use crate::machine_hiding::os_detection::Machine_Hiding::OS_Detection::detect_OS;
            use std::fs;
            pub fn get_permissions(file_path: &str) -> std::io::Result<fs::Permissions>{        
                let os=detect_OS();
                match os {
                    "linux" => {
                        println!("Running on Linux");
                        fs::metadata(file_path).map(|metadata| metadata.permissions()) 
                    },
                    "windows" => {
                        println!("Running on Windows");
                        // Windows 
                    },
                    "macos" => {
                        println!("Running on macOS");
                        // macOS 
                    },
                    _ => {
                        println!("Running on an unknown or unsupported OS: {}", os);
                        // other
                    }
                }
            }

            pub fn set_permissions(file_path: &str, permissions: fs::Permissions) -> std::io::Result<()>{
                let os=detect_OS();
                match os {
                    "linux" => {
                        println!("Running on Linux");
                        fs::set_permissions(file_path, permissions) 
                    },
                    "windows" => {
                        println!("Running on Windows");
                        // Windows 
                    },
                    "macos" => {
                        println!("Running on macOS");
                        // macOS 
                    },
                    _ => {
                        println!("Running on an unknown or unsupported OS: {}", os);
                        // other
                    }
                }
            }
        }
    }
}