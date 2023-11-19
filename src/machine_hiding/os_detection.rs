pub mod Machine_Hiding{
    pub mod  OS_Detection{
        pub fn detect_OS()->&'static str{
            std::env::consts::OS
        }
    }
}