pub mod machine_hiding{
    pub mod  os_detection{
        use std::env;

        pub fn detect_OS()->&'static str{
            std::env::consts::OS
        }

        pub fn pwd()->String{
            env::current_dir().unwrap().into_os_string().into_string().unwrap()
        }
    }
}