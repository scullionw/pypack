#![windows_subsystem = "windows"]

use rust_embed::RustEmbed;
use std::env;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

fn main() {
    let temp_dir = Path::new("temp");

    if temp_dir.exists() {
        fs::remove_dir_all(temp_dir).expect("Could not remove temp directory");
    }

    fs::create_dir(temp_dir).expect("Could not create temp directory");

    let mut exe_path = None;
    for file in Asset::iter() {
        let file_path = Path::new(file.as_ref());

        if file_path.extension().expect("Could not get filename") == "exe" {
            exe_path = Some(file_path.to_path_buf())
        };

        let path = temp_dir.join(file_path);

        let folders = path.parent().expect("Could not get parent");

        if !folders.to_str().expect("Not unicode!").is_empty() {
            fs::create_dir_all(folders)
                .expect("Could not create dirs recursively for embedded files");
        }

        let data = Asset::get(file_path.to_str().expect("File path is not unicode"))
            .expect("Could not get the asset");

        fs::write(path, data).expect("Writing in temp directory failed");
    }

    if let Some(exe_path) = exe_path {
        let current_dir = env::current_dir().expect("Could not get current directory");
        reactor(temp_dir, &exe_path).expect("Reactor execution error");
        env::set_current_dir(current_dir).expect("Could not switch back to original directory");
        fs::remove_dir_all(temp_dir).expect("Could not remove temp directory on shutdown");
    } else {
        eprintln!("No executable found!");
    }
}

fn reactor(temp_dir: &Path, exe_path: &Path) -> Result<(), Box<dyn Error>> {
    env::set_current_dir(temp_dir).expect("Could not change directory");
    let output = Command::new(exe_path).output()?;

    if output.status.success() {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(())
    } else {
        println!("status: {}", output.status);
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Err("reactor command failed.".into())
    }
}
