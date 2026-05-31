////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define a project shell, that is, a REPL/Shell type environment, with an initializer parameter
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::projdef::ProjDef;

type LaunchResult = Result<u32, String>;

pub trait Launcher {
    fn launch(&self, proj: &ProjDef) -> LaunchResult;
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Shell {
    pub exec: String,
    pub params: Vec<String>,
}

pub trait Terminal {
    fn start(&self, shell: &Shell, proj: &ProjDef) -> LaunchResult;
}

pub struct KittyTerm {
    pub bgimage: Option<String>,
}

impl Terminal for KittyTerm {
    fn start(&self, shell: &Shell, proj: &ProjDef) -> LaunchResult {
        match Command::new("kitty")
            .env("PTAG", proj.tag.as_str())
            .env("PROOT", proj.path.as_str())
            .arg(shell.exec.as_str())
            // .args(shell.parms)
            .spawn()
        {
            Ok(p) => Ok(p.id()),
            Err(e) => Err(e.to_string()),
        }
        // Err("Not Implemented".to_string())
    }
}

pub struct TermLauncher {
    pub term: Box<dyn Terminal>,
    pub shell: Box<Shell>,
}

impl Launcher for TermLauncher {
    fn launch(&self, proj: &ProjDef) -> LaunchResult {
        self.term.start(&self.shell, proj)
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn bash_shell() {
//         let shell = ShellDef {
//             name: "Bash".to_string(),
//             exec: "/usr/bin/bash".to_string(),
//             desc: "A Standard Shell".to_string(),
//         };
//         assert_eq!(shell.name, "Bash");
//         assert_eq!(shell.name, "/usr/bin/bash");
//         assert_eq!(shell.name, "A Standard Shell");
//     }
// }
