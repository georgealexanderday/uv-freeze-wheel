use std::{path::PathBuf, process::Command};

use clap::Parser;  

#[derive(Debug, Parser)]  
#[command(about = "Find all files containing a given name.")]  
pub struct Arguments {
    /// Name to find.  // (4)!
    #[arg(short, long)]  
    pub name: String, 
    /// Path to to check.
    #[arg(default_value = ".")] 
    pub path: PathBuf, 
}

// TODO
// unpack built wheel

// create a struct for laoding in the metdata
// req fields: metadata version, name, version
// all others optional but store in struct in hashmap?
// load in the metadata file and store in struct
// capture output of export command
// replace the metdata Requires Dist with the lockfile output
// generate new hashes? - check distinfo


fn export_lock() {
    let _build_result = Command::new("uv")
        .args(["build", "--wheel"])
        .spawn()
        .expect("Failed to build wheel using `uv build --wheel`")
        .wait();

    let export_cmd = Command::new("uv")
        .args([
            "export", 
            "--no-progress", 
            "--no-header", 
            "--no-hashes", 
            "--no-editable", 
            "--no-emit-project", 
            "--locked"
        ])
        .output();

    match export_cmd {
        Ok(output) => {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                println!("{}", stdout);
            }
        }
        Err(e) => {
            eprintln!("Error executing UV export: {}", e);
        }
    }
}






fn main() {
    // let args = Arguments::parse();  
    // println!("{:?}", args);


    export_lock();
}
