use std::io;
use std::io::Write;

use crate::config::*;

pub fn print_welcome(){
    println!(
        r#"
===========================
   TungstenScore {APP_VERSION}
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
        std::io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .unwrap();
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
    {APP_VERSION}
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
