use serde::Deserialize;
use wmi::{COMLibrary, WMIConnection};
use crate::{admin::check_admin, config::*};



#[derive(Debug, Deserialize)]
struct Win32Tpm {
    #[serde(rename = "IsEnabled_InitialValue")]
    is_enabled_initial_value: bool,

    #[serde(rename = "IsActivated_InitialValue")]
    is_activated_initial_value: bool,

    #[serde(rename = "SpecVersion")]
    spec_version: String,
}

const NAMESPACE_PATH: &str = "ROOT\\CIMV2\\Security\\MicrosoftTpm";

pub fn run() -> CheckResult{
    if !check_admin(){
        return CheckResult { name: "TPM 2.0".to_string(), max_score: 0, score: 0, status: CheckStatus::RequiresAdmin }
    }

    let com = match COMLibrary::new(){
        Ok(com) => com,
        Err(_) => return CheckResult { name: "TPM 2.0".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown }
    };
    let wmi = match WMIConnection::with_namespace_path(NAMESPACE_PATH,com){//requires admin
        Ok(wmi) => wmi,
        Err(_) => return CheckResult { name: "TPM 2.0".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown }
    };
    let tpm: Vec<Win32Tpm> = match wmi.raw_query(
        "SELECT IsEnabled_InitialValue, IsActivated_InitialValue, SpecVersion FROM Win32_Tpm"
    ) {
        Ok(tpm) => tpm,
        Err(_) => 
            return CheckResult {name: "TPM 2.0".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown,}
    };

    if tpm.is_empty(){
        return CheckResult { name: "TPM 2.0".to_string(), max_score: 20, score: 0, status: CheckStatus::Disabled }
    }
    
    let tpm_info = &tpm[0];
    if !tpm_info.spec_version.starts_with("2.0")
        || !tpm_info.is_enabled_initial_value
        || !tpm_info.is_activated_initial_value
    {
        CheckResult { name: "TPM 2.0".to_string(), max_score: 20, score: 0, status: CheckStatus::Disabled }
    }else{
        CheckResult { name: "TPM 2.0".to_string(), max_score: 20, score: 20, status: CheckStatus::Enabled }
    }

}