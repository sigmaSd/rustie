use super::Envs;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct Bins {
    bins: HashMap<String, path::PathBuf>,
}

impl Bins {
    pub fn new(envs: &Envs) -> Self {
        let mut bins = HashMap::new();
        if let Some(paths) = envs.get("PATH") {
            let mut paths: Vec<path::PathBuf> = env::split_paths(paths).collect();
            if let Some(rustie_paths) = envs.get("RUSTIE_PATH") {
                paths.extend(env::split_paths(rustie_paths));
            }
            for path in paths {
                if let Ok(entries) = fs::read_dir(path) {
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            let file_name = entry.file_name().into_string().unwrap();
                            bins.insert(file_name, path);
                        }
                    }
                }
            }
        }
        Self { bins }
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.bins.keys()
    }
}
