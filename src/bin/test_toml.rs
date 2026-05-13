use dtool::projdef::ProjDef;

fn main() {
    let name = "Test Project".to_string();
    let path = "/home/pbk/dev/pbk/dtool/test_proj.toml".to_string();
    let pd = ProjDef {
        name: name.clone(),
        path: path.clone(),
    };

    match pd.to_file(path.as_str()) {
        None => println!("Wrote project '{name}' to file '{path}'"),
        Some(msg) => println!("Unable to write project '{name}' to file '{path}': {msg}"),
    };
}
