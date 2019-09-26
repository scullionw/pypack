#![cfg_attr(feature = "nowindow", windows_subsystem = "windows")]

use rust_embed::RustEmbed;
use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

const CREATE_NO_WINDOW: u32 = 0x08000000;

#[derive(RustEmbed)]
#[folder = "dist/"]
struct Asset;

pub struct Packed {
    temp_dir: TempDir,
    exe_path: Option<PathBuf>,
}

impl Packed {
    pub fn new() -> Self {
        Self {
            temp_dir: TempDir::new().expect("Could not create temp dir"),
            exe_path: None,
        }
    }

    fn path(&self) -> &Path {
        self.temp_dir.path()
    }

    pub fn dump(&mut self) {
        for file in Asset::iter() {
            let file_path = Path::new(file.as_ref());

            if file_path.extension().expect("Could not get filename") == "exe" {
                self.exe_path = Some(file_path.to_path_buf())
            };

            let path = self.path().join(file_path);

            let folders = path.parent().expect("Could not get parent");

            if !folders.to_str().expect("Not unicode!").is_empty() {
                fs::create_dir_all(folders)
                    .expect("Could not create dirs recursively for embedded files");
            }

            let data = Asset::get(file_path.to_str().expect("File path is not unicode"))
                .expect("Could not get the asset");

            fs::write(path, data).expect("Writing in temp directory failed");
        }
    }

    pub fn run(&self) {
        if let Some(exe_path) = &self.exe_path {
            execute(self.path(), exe_path);
        } else {
            eprintln!("No executable found!");
        }
    }
}

fn execute(temp_dir: &Path, exe_path: &Path) {
    env::set_current_dir(temp_dir).expect("Could not change directory");
    let mut cmd = Command::new(exe_path);

    if cfg!(feature = "nowindow") {
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let mut child = cmd.spawn().expect("Could not spawn command");
    child.wait().expect("command wasn't running");
}
