use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct VibeConfig {
    pub config_path: String,
    pub prompts_dir: String,
    pub skills_dir: String,
}

impl Default for VibeConfig {
    fn default() -> Self {
        Self {
            config_path: ".vibe/config.toml".to_owned(),
            prompts_dir: ".vibe/prompts".to_owned(),
            skills_dir: ".vibe/skills".to_owned(),
        }
    }
}
