////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Main user data and configuration
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{env, fs};

use serde::{Deserialize, Serialize};

use crate::{projdef::ProjDef, project::ProjData};

use std::fs::write;

const CONFIG_FILE: &str = ".config/dtool/config.toml";
const PROJ_FILE: &str = ".config/dtool/projects.toml";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    term: Option<String>,
}

impl AppConfig {
    pub fn new() -> AppConfig {
        AppConfig { term: None }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjList {
    pub projects: Vec<ProjDef>,
}

impl ProjList {
    pub fn new() -> ProjList {
        ProjList {
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
    pub config: AppConfig,
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

        let proj_file_path = format!(
            "{}/{PROJ_FILE}",
            env::var("HOME").expect("Unable to get home dir")
        );

        let plist = match fs::exists(&proj_file_path).unwrap() {
            true => {
                let proj_file_data = fs::read_to_string(&proj_file_path)
                    .expect(format!("Unable to read file {proj_file_path}").as_str());
                let proj_list: ProjList = toml::from_str(proj_file_data.as_str())
                    .expect(format!("Unable to desrialize AppC").as_str());
                proj_list
            }
            false => ProjList::new(),
        };

        let mut state: AppState = AppState::new(cfg);

        for pd in plist.projects {
            state.projects.push(ProjEntry {
                pdef: pd.clone(),
                proj: load_proj_file(&pd.path),
            });
        }

        Ok(state)
    }

    fn new(cfg: AppConfig) -> AppState {
        AppState {
            projects: Vec::new(),
            config: cfg,
        }
    }

    pub fn sync_projects(&self) {
        let mut projects: Vec<ProjDef> = Vec::new();
        for p in &self.projects {
            projects.push(ProjDef {
                tag: p.pdef.tag.clone(),
                path: p.pdef.path.clone(),
            });
        }

        let proj_list = ProjList { projects };

        let proj_file_path = format!(
            "{}/{PROJ_FILE}",
            env::var("HOME").expect("Unable to get home dir")
        );

        let file_str = toml::to_string(&proj_list).unwrap();
        match write(&proj_file_path, &file_str) {
            Ok(_) => {}
            Err(e) => eprintln!("{e}"),
        };
    }

    pub fn sync_config(&self) {
        let mut projects: Vec<ProjDef> = Vec::new();
        for p in &self.projects {
            projects.push(ProjDef {
                tag: p.pdef.tag.clone(),
                path: p.pdef.path.clone(),
            });
        }

        let config_file_path = format!(
            "{}/{CONFIG_FILE}",
            env::var("HOME").expect("Unable to get home dir")
        );

        let file_str = toml::to_string(&self.config).unwrap();
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
