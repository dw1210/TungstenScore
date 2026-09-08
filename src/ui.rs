use std::io;
use std::io::Write;

use crate::config::*;

pub fn print_welcome(){
    println!(
        r#"
===========================
   TungstenScore v{APP_VERSION}
Windows Security Assessment
===========================
        "#
    );
}

pub fn select_option() -> MenuOption{
    loop{
        println!("\n\n---Menu---");
        println!("1.Scan");
        println!("2.About");
        println!("3.Exit");
        println!("----------");
        print!("Select: ");

        if let Err(e) = std::io::stdout().flush() {
            eprintln!("Failed to flush stdout: {e}");
        }

        let mut input = String::new();

        if io::stdin()
            .read_line(&mut input)
            .is_err(){
                println!("Failed to read input.");
                continue;
            }
        println!();

        match input.trim(){
            "1" => return MenuOption::Scan,
            "2" => return MenuOption::About,
            "3" => return MenuOption::Exit,
            _ => println!("Invalid selection\n"),
        }
    }
}

pub fn print_about(){
    println!(r#"=====About=====
Name:
    TungstenScore
Language: 
    Rust
Description:
    Windows Security Assessment Tool
App version:
    v{APP_VERSION}
Scoring system version:
    {SCORING_SYSTEM_VERSION}
License:
    MIT License
Third-party licenses:
    Third-party licenses are included in THIRD_PARTY.html.
Repository:
    https://github.com/dw1210/TungstenScore
    "#);
}

pub fn print_check_result(check_result: &CheckResult){
    let score: String = match check_result.status{
        CheckStatus::Unknown => String::from("?"),
        CheckStatus::RequiresAdmin => String::from("!"),
        _ => format!("+{}",check_result.score).to_string(),
    };
    println!("{}: {} [{}]",check_result.name, check_result.status.display(), score);
}

pub fn print_final_score(score: f64){
    println!(r#"
--Scan Score--
> {score:.2}/100.00
    "#);
}

pub fn prompt_admin_restart() -> bool{
    loop{
        let mut input = String::new();
        
        print!(
r#"Some features require administrator privileges.
Would you like to restart the app as administrator? [Y/n]"#
        );

        if let Err(e) = std::io::stdout().flush() {
        eprintln!("Failed to flush stdout: {e}");
        }
        if io::stdin()
            .read_line(&mut input)
            .is_err(){
                println!("Failed to read input.");
                continue;
            }
        println!();

        match input.trim(){
            "Y" | "y" | "Yes" | "yes"=> return true,
            "N" | "n" | "No" | "no" => return false,
            _ => println!("Invalid selection\n"),
        };
    }
}

pub fn print_error(error_message: &str){
    eprintln!("Error: {error_message}");
}