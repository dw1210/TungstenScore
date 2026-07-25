pub const APP_VERSION: &str = "v0.4.0";
pub const SCORING_SYSTEM_VERSION: &str = "v0.2.1";

pub enum MenuOption{
    Scan,
    About,
    Exit,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CheckStatus{
    Enabled,
    PartiallyEnabled,
    Disabled,
    Unknown,
}

impl CheckStatus{
    pub fn display(&self) -> String{
        match self{
            CheckStatus::Enabled => "Enabled".to_string(),
            CheckStatus::PartiallyEnabled => "Partially Enabled".to_string(),
            CheckStatus::Disabled => "Disabled".to_string(),
            CheckStatus::Unknown => "Unknown".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult{
    pub name: String,
    pub max_score: u32,
    pub score: u32,
    pub status: CheckStatus,

}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScanSummary{
    pub total_max_score: u32,
    pub total_score: u32,
}