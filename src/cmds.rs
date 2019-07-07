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

impl From<&str> for Cmds {
    fn from(s: &str) -> Cmds {
        use Cmds::*;
        match s {
            "cd" => Cd,
            "exit" => Exit,
            _ => unimplemented!(),
        }
    }
}

impl Cmds {
    pub fn _extract_cmds(s: &str) -> Vec<(usize, String)> {
        let cmds = ["cd", "exit"];
        let mut extracted_cmds = vec![];
        for c in cmds.iter() {
            if let Some(n) = s.find(c) {
                extracted_cmds.push((n, c.to_string()))
            }
        }

        extracted_cmds
    }

    pub fn contains(c: &str) -> bool {
        ["cd", "exit"].contains(&c)
    }

    pub fn get_hint_cond(&self) -> Box<Fn(&str) -> bool> {
        use Cmds::*;
        match self {
            Cd => Box::new(|i: &str| {
                path::Path::new(i.trim_start_matches("cd").trim_start()).is_dir()
            }),
            Exit => Box::new(|_: &str| false),
        }
    }

    pub fn run(&self, s: &[String]) {
        use Cmds::*;
        match self {
            Cd => {
                if s.is_empty() {
                    let _ = env::set_current_dir(dirs::home_dir().unwrap());
                } else {
                    let _ = env::set_current_dir(&s[0]);
                }
            }
            Exit => {
                if s.is_empty() {
                    process::exit(0)
                }
            }
        }
    }
}
