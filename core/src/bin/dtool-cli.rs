////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Command line tool for managing projects
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use core::{
    dtools::{AppState, ProjEntry, load_proj_file},
    projdef::ProjDef,
};
use std::{env, process::exit};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum CmdActionType {
    List,
    Add,
    Promote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum CmdAction {
    List,
    Add(String, String),
    Promote(String),
}

impl CmdAction {
    pub fn execute(&mut self, app: &mut AppState) -> Option<String> {
        match self {
            CmdAction::List => print_proj_list(app),
            CmdAction::Add(t, p) => add_action(app, t, p),
            CmdAction::Promote(t) => promote_action(app, t),
        }
    }
}

pub fn main() {
    let mut app = AppState::load().unwrap();

    let args: Vec<String> = env::args().collect();

    let mut cmd: Option<CmdActionType> = None;

    if args.len() > 1 {
        if &args[1] == "list" {
            // println!("Listing Projects");
            cmd = Some(CmdActionType::List);
        } else if &args[1] == "add" {
            cmd = Some(CmdActionType::Add);
        } else if &args[1] == "promote" {
            cmd = Some(CmdActionType::Promote);
        } else {
            eprintln!("Unknown initial command: '{}'", &args[1]);
            exit(-1);
        }
    }

    match &cmd {
        Some(t) => match read_command(t, args, 2) {
            Ok(mut c) => match c.execute(&mut app) {
                Some(e) => {
                    eprintln!("Error running [{t:?}] -- {e}");
                    exit(-1);
                }
                None => {}
            },
            Err(s) => {
                eprintln!("Unable to read {cmd:?} command from parameters: {s}");
            }
        },
        None => print_help(&args[1]),
    }
}

fn print_help(name: &String) {
    println!("{name}: A Development tool");
}

fn read_command(
    ctype: &CmdActionType,
    args: Vec<String>,
    index: usize,
) -> Result<CmdAction, String> {
    match ctype {
        CmdActionType::List => read_list_cmd(args, index),
        CmdActionType::Add => read_add_cmd(args, index),
        CmdActionType::Promote => read_promote_cmd(args, index),
    }
}

fn read_list_cmd(args: Vec<String>, index: usize) -> Result<CmdAction, String> {
    Ok(CmdAction::List)
}

fn read_add_cmd(args: Vec<String>, index: usize) -> Result<CmdAction, String> {
    let mut tag: Option<String> = None;
    let mut path: Option<String> = None;

    let mut i = index;
    while i < args.len() {
        if &args[i] == "--help" {
            eprintln!("Help Not Implemented Yet");
        } else {
            match tag {
                None => {
                    tag = Some(args[i].clone());
                }
                Some(_) => match path {
                    None => {
                        path = Some(args[i].clone());
                    }
                    Some(_) => {
                        panic!("Unexpected parameter: {}", args[i]);
                    }
                },
            }
        }
        i = i + 1;
    }

    match (tag, path) {
        (Some(t), Some(p)) => Ok(CmdAction::Add(t, p)),
        (Some(t), None) => Ok(CmdAction::Add(
            t,
            env::var("PWD").expect("Unable to get current working directory"),
        )),
        (None, None) => Err(format!("At least a tag is required")),
        _ => panic!("Can't get here"),
    }
}

fn read_promote_cmd(args: Vec<String>, index: usize) -> Result<CmdAction, String> {
    let mut tag: Option<String> = None;

    let mut i = index;
    while i < args.len() {
        if &args[i] == "--help" {
            eprintln!("Help Not Implemented Yet");
        } else {
            match tag {
                None => {
                    tag = Some(args[i].clone());
                }
                Some(_) => {
                    panic!("Unexpected parameter: {}", args[i]);
                }
            }
        }
        i = i + 1;
    }

    match tag {
        Some(t) => Ok(CmdAction::Promote(t)),
        None => Err(format!("A tag is required")),
    }
}

fn print_proj_list(app: &AppState) -> Option<String> {
    for pe in &app.projects {
        println!(
            "{} {}",
            pe.pdef.tag,
            match &pe.proj {
                Ok(p) => p.name.clone(),
                Err(s) => s.clone(),
            }
        );
    }

    None
}

fn add_action(app: &mut AppState, tag: &String, path: &String) -> Option<String> {
    for p in &app.projects {
        if &p.pdef.tag == tag {
            panic!("add_action: tag [{tag}] already points to {}", p.pdef.path)
        } else {
            if &p.pdef.path == path {
                panic!("add_action: tag [{tag}] already points to {}", p.pdef.path)
            }
        }
    }

    let entry = ProjEntry {
        pdef: ProjDef {
            tag: tag.clone(),
            path: path.clone(),
        },
        proj: load_proj_file(path),
    };

    app.projects.push(entry);

    app.sync();

    None
}

fn promote_action(app: &AppState, tag: &String) -> Option<String> {
    Some(format!("apromote_action: Not Implemented Yet"))
}
