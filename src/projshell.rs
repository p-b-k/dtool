////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Define a project shell, that is, a REPL/Shell type environment, with an initializer parameter
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ShellDef {
    name: String,
    exec: String,
    desc: String,
}

pub struct LaunchResult {
    pub pid: usize,
    pub class: String,
}

pub trait Launcher {
    fn launch() -> LaunchResult;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bash_shell() {
        let shell = ShellDef {
            name: "Bash".to_string(),
            exec: "/usr/bin/bash".to_string(),
            desc: "A Standard Shell".to_string(),
        };
        assert_eq!(shell.name, "Bash");
        assert_eq!(shell.name, "/usr/bin/bash");
        assert_eq!(shell.name, "A Standard Shell");
    }
}
