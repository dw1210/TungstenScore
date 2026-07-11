use std::io;
use std::io::Write;

const VERSION: &str = "v0.1.0";

enum MenuOption{
    Scan,
    About,
    Exit,
}

fn main() {
    print_welcome();
    loop{
        

        match select_option() {
            MenuOption::Scan => run_scan(),
            MenuOption::About => print_about(),
            MenuOption::Exit => break,
        }
    }
}

fn print_welcome(){
    println!(
        r#"
===========================
   TungstenScore {VERSION}
Windows Security Assessment
===========================
        "#
    );
}

fn select_option() -> MenuOption{
    loop{
        println!("---Menu---");
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

fn print_about(){
    println!(r#"=====About=====
Name:
    TungstenScore
Language: 
    Rust
Description:
    Windows Security Assessment Tool
Version:
    {VERSION}
License:
    MIT Licence
    "#);
}

fn run_scan(){
    todo!("run scan");
}

