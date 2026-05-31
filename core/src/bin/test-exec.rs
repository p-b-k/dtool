use core::{
    projdef::ProjDef,
    projshell::{KittyTerm, Launcher, Shell, TermLauncher},
};

fn main() {
    let t = KittyTerm { bgimage: None };

    let s = Shell {
        exec: "nu".to_string(),
        params: Vec::new(),
    };

    let l = TermLauncher {
        term: Box::new(t),
        shell: Box::new(s),
    };

    let p = ProjDef {
        tag: "test".to_string(),
        path: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
    };

    match l.launch(&p) {
        Ok(pid) => println!("Started with process {pid}"),
        Err(e) => println!("Error: {e:?}"),
    }
}
