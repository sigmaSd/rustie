use super::utils::{self, StringTools};
use super::Cmds;
use std::io;
use std::process;

impl super::Rustie {
    pub fn eval(&mut self) -> io::Result<()> {
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

    fn parse_as_intern_cmd(&self, tokens: Vec<String>) -> io::Result<()> {
        let cmd = Cmds::from(tokens[0].as_str());
        cmd.run(&tokens[1..]);

        Ok(())
    }

    fn parse_as_extern_cmd(&self, tokens: Vec<String>) -> io::Result<()> {
        utils::disable_raw_mode();

        process::Command::new(&tokens[0])
            .args(&tokens[1..])
            .spawn()?
            .wait()?;

        Ok(())
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
}
