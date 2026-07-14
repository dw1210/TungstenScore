use std::io;
use std::io::Write;
use winreg::RegKey;
use winreg::enums::*;

const APP_VERSION: &str = "v0.2.1";
const SCORE_SYSTEM_VERSION: &str = "v0.1.0";

enum MenuOption{
    Scan,
    About,
    Exit,
}

enum UacStatus{
    Enabled,
    Disabled,
    Unknown,
}

struct ScanScore{
    uac: i32,
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
   TungstenScore {APP_VERSION}
Windows Security Assessment
===========================
        "#
    );
}

fn _exit_program(is_err: bool, msg: &str, code: i32) -> !{
    if !is_err{
        println!("{}", msg);
    }else{
        eprintln!("{}", msg);
    }
    std::process::exit(code);
}


fn select_option() -> MenuOption{
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

fn print_about(){
    println!(r#"=====About=====
Name:
    TungstenScore
Language: 
    Rust
Description:
    Windows Security Assessment Tool
App version:
    {APP_VERSION}
Score system version:
    {SCORE_SYSTEM_VERSION}
License:
    MIT License
Third-party licenses:
    Third-party licenses are included in THIRD_PARTY.html.
    "#);
}

fn check_uac() -> UacStatus{
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);

    let key = match hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System"){
        Ok(key) => key,
        Err(_) => return UacStatus::Unknown,
    };

    let enable_lua: u32 = match key.get_value("EnableLUA"){
        Ok(value) => value,
        Err(_) => return UacStatus::Unknown,
    };

    match enable_lua {
        1 => UacStatus::Enabled,
        0 => UacStatus::Disabled,
        _ => UacStatus::Unknown,
    }

}

fn run_scan(){
    println!("\nScan Started.\n");
    let mut max_score = 0;
    let mut scores = ScanScore{
        uac: 20,
    };

    //uac
    scores.uac = match check_uac() {
        UacStatus::Enabled => {
            max_score += 20;
            println!("UAC: Enabled [+20]");
            20
        },
        UacStatus::Disabled => {
            max_score += 20;
            println!("UAC: disabled [!]");
            0
        },
        UacStatus::Unknown => {
            println!("UAC: Unknown [N/A]");
            0
        },
    };

    //total score
    let total_score: f64 = if max_score != 0 {
        ((scores.uac) as f64 / max_score as f64) * 100_f64
    }else {
        0.0
    };
    println!("--- Total score: {:.2}/100 ---", total_score);

}

