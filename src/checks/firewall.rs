use windows_firewall::{get_firewall_state, Profile};
use crate::config::*;

const FIREWALL_PROFILES: [Profile; 2] = [
    Profile::Private,
    Profile::Public,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum FirewallProfileStatus{
    Enabled,
    Disabled,
    Unknown,
}

pub fn run() -> CheckResult{
    let mut firewall_profile_status: [FirewallProfileStatus; 2] = [const {FirewallProfileStatus::Unknown}; 2];
    let mut profile_count: usize = 0;
    for profile in FIREWALL_PROFILES {
        if profile_count >= 2{
            break;
        }
        firewall_profile_status[profile_count] = match get_firewall_state(profile){
            Ok(true) => FirewallProfileStatus::Enabled,
            Ok(false) => FirewallProfileStatus::Disabled,
            Err(_) => FirewallProfileStatus::Unknown, 
        };
        profile_count += 1;
    }

    if firewall_profile_status.iter().all(|&profile| firewall_profile_status[0] == profile){
        if firewall_profile_status[0] == FirewallProfileStatus::Enabled{
            CheckResult { name: "Firewall".to_string(), max_score: 20, score: 20, status: CheckStatus::Enabled }
        }else if firewall_profile_status[0] == FirewallProfileStatus::Disabled {
            CheckResult { name: "Firewall".to_string(), max_score: 20, score: 0, status: CheckStatus::Disabled }
        }else{
            CheckResult { name: "Firewall".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown }
        }
    }else if !firewall_profile_status.contains(&FirewallProfileStatus::Unknown){
        CheckResult { name: "Firewall".to_string(), max_score: 20, score: 10, status: CheckStatus::PartiallyEnabled }
    }else{
        CheckResult { name: "Firewall".to_string(), max_score: 0, score: 0, status: CheckStatus::Unknown }
    }
    
}
