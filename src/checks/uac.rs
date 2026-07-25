use winreg::RegKey;
use winreg::enums::*;
use crate::config::*;

const HKLM: RegKey = RegKey::predef(HKEY_LOCAL_MACHINE);
const UAC_VALUE_LOCATION: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Policies\\System";

pub fn run() -> CheckResult{
    let key = match HKLM.open_subkey(UAC_VALUE_LOCATION){
        Ok(key) => key,
        Err(_) => return CheckResult { name: "UAC".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown },
    };

    let enable_lua: u32 = match key.get_value("EnableLUA"){
        Ok(value) => value,
        Err(_) => return CheckResult { name: "UAC".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown },
    };

    match enable_lua {
        1 => CheckResult { name: "UAC".to_string(), max_score: 20, score: 20, status: CheckStatus::Enabled },
        0 => CheckResult { name: "UAC".to_string(), max_score: 20, score: 0, status: CheckStatus::Disabled },
        _ => CheckResult { name: "UAC".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown },
    }
}