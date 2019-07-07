use super::utils::{self, StringTools};
use super::Cmds;
use std::io;
use std::process;

impl super::Rustie {
    pub fn eval(&mut self) -> io::Result<()> {
        for cmd in self.buffer.split_cmds() {
            let tokens = cmd.split_tokens();
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
}
