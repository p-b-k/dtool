////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Main user data and configuration
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::{projdef::ProjDef, project::ProjData};

const CONFIG_FILE: &str = ".config/dtool/config.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub projects: Vec<ProjDef>,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig {
            projects: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjEntry {
    pub pdef: ProjDef,
    pub proj: Result<ProjData, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub projects: Vec<ProjEntry>,
}

impl AppState {
    pub fn load() -> Result<AppState, String> {
        let config_file_path = format!(
            "{}/{CONFIG_FILE}",
            env::var("HOME").expect("Unable to get home dir")
        );

        let cfg = match fs::exists(&config_file_path).unwrap() {
            true => {
                let config_file_data = fs::read_to_string(&config_file_path)
                    .expect(format!("Unable to read file {config_file_path}").as_str());
                let app_config: AppConfig = toml::from_str(config_file_data.as_str())
                    .expect(format!("Unable to desrialize AppC").as_str());
                app_config
            }
            false => AppConfig::new(),
        };

        let mut state: AppState = AppState::new();

        for pd in cfg.projects {
            state.projects.push(ProjEntry {
                pdef: pd.clone(),
                proj: load_proj_file(&pd.path),
            });
        }

        Ok(state)
    }

    fn new() -> AppState {
        AppState {
            projects: Vec::new(),
        }
    }
}

fn load_proj_file(path: &String) -> Result<ProjData, String> {
    Err("load_proj_file: Not Implemented Yet".to_string())
}
