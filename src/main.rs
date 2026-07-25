mod config;
mod checks;
mod scanner;
mod ui;

use crate::config::*;
use crate::scanner::*;
use crate::ui::*;

fn main() {
    print_welcome();
    loop{
        match select_option() {
            MenuOption::Scan => perform_scan_and_print(),
            MenuOption::About => print_about(),
            MenuOption::Exit => break,
        }
    }
}
