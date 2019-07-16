use super::utils::{self, StringTools};
use super::Cmds;
use std::io::{self, Read};
use std::process;

impl super::Rustie {
    pub fn eval(&mut self) -> io::Result<()> {
        if let Ok(res) = Self::try_eval_as_math(&self.buffer) {
            self.print(res, crossterm::Color::Magenta);
            self.new_line();
            return Ok(());
        }

        for cmd in self.buffer.split_cmds() {
            let mut tokens = cmd.split_tokens();
            self.replace_vars(&mut tokens);

            if Cmds::contains(&tokens[0]) {
                self.parse_as_intern_cmd(tokens)?
            } else {
                self.parse_as_extern_cmd(tokens)?
            }
        }

        Ok(())
    }

    fn try_eval_as_math(e: &str) -> Result<evalexpr::Value, evalexpr::EvalexprError> {
        evalexpr::eval(e)
    }

    fn replace_vars(&self, v: &mut [String]) {
        v.iter_mut().for_each(|s| {
            if s.starts_with('$') {
                if let Some(hit) = self.envs.get(&s[1..]) {
                    *s = hit.to_string();
                }
            }
        })
    }

    fn parse_as_intern_cmd(&self, tokens: Vec<String>) -> io::Result<()> {
        let cmd = Cmds::from(tokens[0].as_str());
        cmd.run(&tokens[1..]);

        Ok(())
    }

    fn parse_as_extern_cmd(&mut self, tokens: Vec<String>) -> io::Result<()> {
        utils::disable_raw_mode();

        if tokens.contains(&"|".into()) {
            self.print(
                Self::run_with_pipes(&tokens).unwrap(),
                crossterm::Color::Cyan,
            );
        } else {
            process::Command::new(&tokens[0])
                .args(&tokens[1..])
                .spawn()?
                .wait()?;
        }

        Ok(())
    }

    fn run_with_pipes(cmd: &[String]) -> io::Result<String> {
        let mut mem = None;

        let runit = |mem: Option<process::Child>, c: &[String]| -> Option<process::Child> {
            if c.is_empty() {
                return None;
            }

            let mut c = c.iter();

            let mem = if mem.is_some() {
                mem.unwrap().stdout
            } else {
                None
            };

            if let Some(mem) = mem {
                if let Ok(child) = process::Command::new(c.next()?)
                    .args(&c.collect::<Vec<&String>>())
                    .stdin(mem)
                    .stdout(process::Stdio::piped())
                    .spawn()
                {
                    Some(child)
                } else {
                    None
                }
            } else if let Ok(child) = process::Command::new(c.next()?)
                .args(&c.collect::<Vec<&String>>())
                .stdout(process::Stdio::piped())
                .spawn()
            {
                Some(child)
            } else {
                None
            }
        };

        for c in cmd.split(|c| c == "|") {
            mem = runit(mem, c);
        }

        let mut s = String::new();
        if let Some(mem) = mem {
            mem.stdout.unwrap().read_to_string(&mut s)?;
        }
        Ok(s)
    }
}
