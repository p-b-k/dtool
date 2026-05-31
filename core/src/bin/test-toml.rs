use core::projdef::ProjDef;

fn main() {
    let tag = "test".to_string();
    let path = "/home/pbk/dev/pbk/dtool/test_proj.toml".to_string();

    let pd = ProjDef {
        tag: tag.clone(),
        path: path.clone(),
    };

    match pd.to_file(path.as_str()) {
        None => println!("Wrote project '{tag}' to file '{path}'"),
        Some(msg) => println!("Unable to write project '{tag}' to file '{path}': {msg}"),
    };
}
