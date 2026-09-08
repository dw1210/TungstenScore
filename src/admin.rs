use std::process::Command;
use crate::ui::*;

fn restart_as_admin() -> Result<(), std::io::Error>{

    
    let exe = match std::env::current_exe(){
        Ok(exe) => exe,
        Err(msg) => return Err(msg),
    };
    match Command::new("powershell")
        .args([
            "-Command",
            &format!(
                "Start-Process '{}' -Verb RunAs",
                exe.display()
            ),
        ])
        .spawn(){
        Ok(_) => Ok(()),
        Err(msg) => {print_error(&msg.to_string()); Err(msg)},
    }
}

pub fn check_admin() -> bool  {
    is_elevated::is_elevated()
}

pub fn request_admin_restart() -> bool{
    if !check_admin(){
        if !prompt_admin_restart(){
            return false;
        }
        match restart_as_admin(){
            Ok(()) => true,
            Err(msg) => {print_error(&msg.to_string()); false},
        }
    }else{
        false
    }
}
