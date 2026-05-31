////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Main user data and configuration
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::{projdef::ProjDef, project::ProjData};

use std::fs::write;

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

    pub fn sync(&self) {
        let mut projects: Vec<ProjDef> = Vec::new();
        for p in &self.projects {
            projects.push(ProjDef {
                tag: p.pdef.tag.clone(),
                path: p.pdef.path.clone(),
            });
        }

        let config = AppConfig { projects };

        let config_file_path = format!(
            "{}/{CONFIG_FILE}",
            env::var("HOME").expect("Unable to get home dir")
        );

        let file_str = toml::to_string(&config).unwrap();
        match write(&config_file_path, &file_str) {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        };
    }
}

pub fn load_proj_file(path: &String) -> Result<ProjData, String> {
    match fs::exists(&path).unwrap() {
        true => {
            let data =
                fs::read_to_string(&path).expect(format!("Unable to read file {path}").as_str());
            let proj_data: ProjData = toml::from_str(data.as_str())
                .expect(format!("Unable to desrialize Project").as_str());
            Ok(proj_data)
        }
        false => Err(format!("load_proj_file: file ({path}) does not exist")),
    }
}
