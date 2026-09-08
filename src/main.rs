mod config;
mod checks;
mod scanner;
mod ui;
mod admin;

use crate::config::*;
use crate::scanner::*;
use crate::ui::*;
use crate::admin::*;

fn main() {
    print_welcome();
    if request_admin_restart(){
        return;
    }
    loop{
        match select_option() {
            MenuOption::Scan => perform_scan_and_print(),
            MenuOption::About => print_about(),
            MenuOption::Exit => break,
        }
    }
}
