use super::utils::{self, StringTools};
use super::Cmds;
use std::io;
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
            Self::run_with_pipes(&tokens).unwrap();
        } else {
            process::Command::new(&tokens[0])
                .args(&tokens[1..])
                .spawn()?
                .wait()?;
        }

        Ok(())
    }

    fn run_with_pipes(cmd: &[String]) -> io::Result<()> {
        let mut stdin = None;

        let runit = |stdin: Option<process::Child>,
                     stdout: process::Stdio,
                     cmd: &[String]|
         -> Option<process::Child> {
            if cmd.is_empty() {
                return None;
            }

            let mut cmd = cmd.iter();

            let stdin = if stdin.is_some() {
                stdin.unwrap().stdout
            } else {
                None
            };

            if let Some(stdin) = stdin {
                if let Ok(child) = process::Command::new(cmd.next()?)
                    .args(&cmd.collect::<Vec<&String>>())
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn()
                {
                    Some(child)
                } else {
                    None
                }
            } else if let Ok(child) = process::Command::new(cmd.next()?)
                .args(&cmd.collect::<Vec<&String>>())
                .stdout(stdout)
                .spawn()
            {
                Some(child)
            } else {
                None
            }
        };

        let mut cmd = cmd.split(|c| c == "|").peekable();
        while let Some(c) = cmd.next() {
            let stdout = if cmd.peek().is_some() {
                process::Stdio::piped()
            } else {
                process::Stdio::inherit()
            };
            stdin = runit(stdin, stdout, c);
        }
        // wait for the last command
        if let Some(process) = stdin.as_mut() {
            let _ = process.wait();
        }

        Ok(())
    }
}
