////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Command line tool for managing projects
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{env, process::exit};

use core::dtools::AppState;

use cli::command::{CmdAction, execute};

pub fn main() {
    env_logger::init();

    let mut app = AppState::load().unwrap();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!(
            "At least one command is required (try running {} --help)",
            args[0],
        );
        exit(-1);
    }

    match CmdAction::from_args(args[1].as_str(), &args, 2) {
        Ok(c) => match execute(&c, &mut app) {
            Some(e) => {
                eprintln!("Error running [{c:?}] -- {e}");
                exit(-1);
            }
            None => {
                app.sync_config();
                app.sync_projects();
            }
        },
        Err(s) => {
            eprintln!("Unable to read {:?} command from parameters: {s}", args[1]);
            exit(-1);
        }
    }
}
