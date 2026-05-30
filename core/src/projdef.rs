////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define a project def structure
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, write};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProjDef {
    pub name: String,
    pub path: String,
}

impl ProjDef {
    pub fn from_file(file_path: &str) -> Result<ProjDef, String> {
        match read_to_string(file_path) {
            Ok(s) => ProjDef::from_str(&s),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn from_str(data: &String) -> Result<ProjDef, String> {
        match toml::from_str::<ProjDef>(data) {
            Ok(pd) => Ok(pd),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn to_file(&self, file_path: &str) -> Option<String> {
        let file_str = toml::to_string(self).unwrap();
        match write(file_path, file_str) {
            Ok(_) => None,
            Err(e) => Some(e.to_string()),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn it_works() {
//         let result = ProjDef::from_str("")
//         assert_eq!(result, 4);
//     }
// }
