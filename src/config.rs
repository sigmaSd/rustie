use super::Envs;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

#[derive(Debug)]
enum Errors {
    ParseVarError,
}

enum Cmds {
    SETUVAR,
}

impl FromStr for Cmds {
    type Err = Errors;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "SETUVAR" => Ok(Cmds::SETUVAR),
            _ => Err(Errors::ParseVarError),
        }
    }
}

pub struct Config {}

impl Config {
    pub fn parse_config(envs: &mut Envs) {
        let config_dir = dirs::config_dir().unwrap().join("rustie");
        let config_file = config_dir.join("rustie.config");
        let _ = fs::create_dir_all(&config_dir);
        let data = fs::read_to_string(&config_file).unwrap_or_default();

        let mut config_evns = HashMap::new();

        for line in data.lines() {
            Self::parse(&mut config_evns, line)
                .unwrap_or_else(|_| eprintln!("Error while parsing line: {}", line));
        }

        Self::fusion_map(envs.get_mut_map(), config_evns);
    }

    fn parse(envs: &mut HashMap<String, String>, s: &str) -> Result<(), Errors> {
        let mut s = s.split_whitespace();
        match Cmds::from_str(s.next().unwrap())? {
            Cmds::SETUVAR => {
                envs.insert(s.next().unwrap().to_string(), s.next().unwrap().to_string());
            }
        }

        Ok(())
    }

    fn fusion_map(h1: &mut HashMap<String, String>, h2: HashMap<String, String>) {
        for (k, v) in h2.into_iter() {
            h1.insert(k, v);
        }
    }
}
