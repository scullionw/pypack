#![cfg_attr(feature = "nowindow", windows_subsystem = "windows")]

pub use rust_embed;

use rust_embed::RustEmbed;
use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

const CREATE_NO_WINDOW: u32 = 0x08000000;

fn execute(temp_dir: &Path, exe_path: &Path) {
    env::set_current_dir(temp_dir).expect("Could not change directory");
    let mut cmd = Command::new(exe_path);

    if cfg!(feature = "nowindow") {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd.spawn().expect("Could not spawn command");
    child.wait().expect("command wasn't running");
}

pub trait RustEmbedExt {
    fn dump_and_exec();
}

impl<T: RustEmbed> RustEmbedExt for T {
    fn dump_and_exec() {
        let mut exe_path = None;
        let temp_dir = TempDir::new().expect("Could not create temp dir");
        for file in T::iter() {
            let file_path = Path::new(file.as_ref());

            if file_path.extension().expect("Could not get filename") == "exe" {
                exe_path = Some(file_path.to_path_buf())
            };

            let path = temp_dir.path().join(file_path);

            let folders = path.parent().expect("Could not get parent");

            if !folders.to_str().expect("Not unicode!").is_empty() {
                fs::create_dir_all(folders)
                    .expect("Could not create dirs recursively for embedded files");
            }

            let data = T::get(file_path.to_str().expect("File path is not unicode"))
                .expect("Could not get the asset");

            fs::write(path, data).expect("Writing in temp directory failed");
        }
        if let Some(exe_path) = &exe_path {
            execute(temp_dir.path(), exe_path);
        } else {
            eprintln!("No executable found!");
        }
    }
}
