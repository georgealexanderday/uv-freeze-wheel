use std::{fs, path,io, process::Command};
use zip::ZipArchive;

use clap::Parser;  

#[derive(Debug, Parser)]  
#[command(about = "Find all files containing a given name.")]  
pub struct Arguments {
    /// Name to find.  // (4)!
    #[arg(short, long)]  
    pub name: String, 
    /// Path to to check.
    #[arg(default_value = ".")] 
    pub path: path::PathBuf, 
}


const DIST_DIR: &str = "./dist";

// TODO
// unpack built wheel

// create a struct for laoding in the metdata
// req fields: metadata version, name, version
// all others optional but store in struct in hashmap?
// load in the metadata file and store in struct
// capture output of export command
// replace the metdata Requires Dist with the lockfile output
// generate new hashes? - check distinfo
//

fn build_wheel() {
    if fs::metadata(DIST_DIR).is_ok() {
        let _ = fs::remove_dir_all(DIST_DIR);
    }

    let _ = Command::new("uv")
        .args(["build", "--wheel"])
        .status()
        .expect("Failed to build wheel using `uv build --wheel`");
}

fn unpack_wheel() -> io::Result<()> {
    // Find wheel file
    let wheel_path = fs::read_dir(DIST_DIR)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|p| p.extension().unwrap_or_default() == "whl")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Wheel file not found"))?;

    // Open and extract wheel
    let file = fs::File::open(&wheel_path)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(path::Path::new(DIST_DIR))?;
    Ok(())
}




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

    build_wheel();
    let _ = unpack_wheel();
    export_lock();
}
