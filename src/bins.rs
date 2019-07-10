use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;

#[derive(Debug)]
pub struct Bins {
    bins: HashMap<String, path::PathBuf>,
}

impl Bins {
    pub fn new() -> Self {
        let mut bins = HashMap::new();
        if let Some((_, paths)) = env::vars()
            .filter(|(s, _)| s == "PATH")
            .collect::<Vec<(String, String)>>()
            .get(0)
        {
            let paths: Vec<&str> = paths.split(':').collect();
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
}
