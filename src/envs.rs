use std::collections::HashMap;
use std::env;
use std::path;

pub struct Envs {
    envs: HashMap<String, String>,
}

impl Envs {
    pub fn new() -> Self {
        Self {
            envs: env::vars().collect(),
        }
    }

    pub fn get_mut_map(&mut self) -> &mut HashMap<String, String> {
        &mut self.envs
    }

    pub fn get(&self, s: &str) -> Option<&String> {
        self.envs.get(s)
    }

    pub fn keys(&self) -> Vec<String> {
        self.envs.keys().map(ToOwned::to_owned).collect()
    }

    pub fn update_os_path(&mut self) {
        let mut paths: Vec<path::PathBuf> =
            env::split_paths(self.envs.get("PATH").unwrap()).collect();
        if let Some(rustie_paths) = self.envs.get("RUSTIE_PATH") {
            paths.extend(env::split_paths(rustie_paths));

            *self.envs.get_mut("PATH").unwrap() =
                env::join_paths(&paths).unwrap().into_string().unwrap();
            env::set_var("PATH", env::join_paths(&paths).unwrap());
        }
    }
}
