use std::collections::HashMap;
use std::env;

pub struct Envs {
    envs: HashMap<String, String>,
}

impl Envs {
    pub fn new() -> Self {
        Self {
            envs: env::vars().collect(),
        }
    }

    pub fn get(&self, s: &str) -> Option<&String> {
        self.envs.get(s)
    }
}
