use super::utils::StringTools;
use std::env;
use std::path;
use std::process;

pub enum Cmds {
    Cd,
    Exit,
}

impl ToString for Cmds {
    fn to_string(&self) -> String {
        use Cmds::*;
        match self {
            Cd => "cd".into(),
            Exit => "exit".into(),
        }
    }
}

impl From<String> for Cmds {
    fn from(s: String) -> Cmds {
        use Cmds::*;
        match s.as_str() {
            "cd" => Cd,
            "exit" => Exit,
            _ => unimplemented!(),
        }
    }
}

impl Cmds {
    pub fn extract_cmds(s: &str) -> Vec<(usize, String)> {
        let cmds = ["cd", "exit"];
        let mut extracted_cmds = vec![];
        for c in cmds.iter() {
            if let Some(n) = s.find(c) {
                extracted_cmds.push((n, c.to_string()))
            }
        }

        extracted_cmds
    }

    pub fn get_hint_cond(&self) -> Box<Fn(&str) -> bool> {
        use Cmds::*;
        match self {
            Cd => Box::new(|i: &str| path::Path::new(i).is_dir()),
            Exit => Box::new(|_: &str| false),
        }
    }

    pub fn run(&self, s: &str) {
        use Cmds::*;
        match self {
            Cd => env::set_current_dir(s.to_owned().split_as_cmd().next().unwrap()).unwrap(),
            Exit => {
                if s.is_empty() {
                    process::exit(0)
                }
            }
        }
    }
}
