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
    fn run();
}

impl<T: RustEmbed> RustEmbedExt for T {
    fn run() {
        let mut packed = Packed::<T>::new();
        packed.dump();
        packed.launch();
    }
}

struct Packed<T: RustEmbed> {
    temp_dir: tempfile::TempDir,
    exe_path: ::std::option::Option<::std::path::PathBuf>,
    marker: std::marker::PhantomData<T>,
}

impl<T: RustEmbed> Packed<T> {
    fn new() -> Self {
        Self {
            temp_dir: tempfile::TempDir::new().expect("Could not create temp dir"),
            exe_path: None,
            marker: std::marker::PhantomData,
        }
    }

    fn dump(&mut self) {
        for file in T::iter() {
            let file_path = ::std::path::Path::new(file.as_ref());

            if file_path.extension().expect("Could not get filename") == "exe" {
                self.exe_path = Some(file_path.to_path_buf())
            };

            let path = self.temp_dir.path().join(file_path);

            let folders = path.parent().expect("Could not get parent");

            if !folders.to_str().expect("Not unicode!").is_empty() {
                ::std::fs::create_dir_all(folders)
                    .expect("Could not create dirs recursively for embedded files");
            }

            let data = T::get(file_path.to_str().expect("File path is not unicode"))
                .expect("Could not get the asset");

            ::std::fs::write(path, data).expect("Writing in temp directory failed");
        }
    }

    fn launch(&self) {
        if let Some(exe_path) = &self.exe_path {
            execute(self.temp_dir.path(), exe_path);
        } else {
            eprintln!("No executable found!");
        }
    }
}
