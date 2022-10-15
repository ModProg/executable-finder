use std::{env::VarError, fs, os::unix::fs::PermissionsExt};

use crate::Executable;

pub fn split_paths(path: &str) -> impl Iterator<Item = &str> {
    path.split(':')
}

pub fn search_dir() -> Result<fn(&str) -> Option<Vec<Executable>>, VarError> {
    Ok(|path: &str| -> Option<Vec<Executable>> {
        let mut exes = Vec::new();
        if let Ok(dir) = fs::read_dir(path) {
            for entry in dir.flatten() {
                // We need to call metadata on the path to follow symbolic links
                if let Ok(metadata) = entry.path().metadata() {
                    if !metadata.is_file() {
                        continue;
                    }

                    let path = entry.path();
                    if let Some(filename) = path.file_name() {
                        let permissions = metadata.permissions();
                        if permissions.mode() & 0o111 != 0 {
                            let exe = Executable {
                                name: filename.to_string_lossy().to_string(),
                                path,
                            };

                            exes.push(exe);
                        }
                    }
                }
            }
        }

        if exes.is_empty() {
            None
        } else {
            Some(exes)
        }
    })
}
