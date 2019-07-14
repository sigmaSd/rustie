use super::Envs;
use std::collections::HashMap;
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
            let mut paths: Vec<&str> = paths.split(':').collect();
            if let Some(rustie_paths) = envs.get("RUSTIE_PATH") {
                paths.extend(rustie_paths.split(':'));
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

    pub fn keys(&self) -> Vec<String> {
        self.bins.keys().map(ToOwned::to_owned).collect()
    }

    pub fn get_cmd(&self, c: &str) -> Option<&path::PathBuf> {
        self.bins.get(c)
    }
}
