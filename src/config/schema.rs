use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RepoDefinition {
    pub name: String,
    pub enabled: bool,
    pub baseurl: Option<String>,
    pub metalink: Option<String>,
    pub mirrorlist: Option<String>,
    pub gpgcheck: bool,
    pub gpgkey: Option<Vec<String>>,
    pub priority: Option<u32>,
}

impl RepoDefinition {
    pub fn validate(&self) -> Result<(), String> {
        if self.baseurl.is_none() && self.metalink.is_none() && self.mirrorlist.is_none() {
            return Err(format!(
                "Repository '{}' must specify at least one of: baseurl, metalink, mirrorlist",
                self.name
            ));
        }

        if self.gpgcheck && self.gpgkey.is_none() {
            return Err(format!(
                "Repository '{}' has gpgcheck enabled but no gpgkey specified",
                self.name
            ));
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HistorySchema {
    pub id: i64,
    pub timestamp: String,
    pub user_id: u32,
    pub command: String,
    pub return_code: i32,
    pub packages_altered: Vec<String>,
}
