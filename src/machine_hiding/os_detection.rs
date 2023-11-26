use std::env;
use std::path::Path;
use std::io;

#[allow(non_snake_case)]
pub fn detect_OS()->&'static str{
    std::env::consts::OS
}

pub fn pwd()->String{
    env::current_dir().unwrap().into_os_string().into_string().unwrap()
}

pub fn pwd_move(target: &str) -> io::Result<()> {
    let current_dir = env::current_dir()?;

    let new_dir = match target {
        "." => current_dir,
        ".." => current_dir.parent().unwrap_or(&current_dir).to_path_buf(),
        _ => Path::new(target).to_path_buf(),
    };

    env::set_current_dir(&new_dir)?;
    Ok(())
}