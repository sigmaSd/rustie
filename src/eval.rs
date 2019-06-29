use super::utils::StringTools;
use super::Cmds;
use std::io;
use std::process;

impl super::Rustie {
    pub fn eval(&mut self) -> io::Result<()> {
        let cmds = Cmds::extract_cmds(&self.buffer);
        if cmds.is_empty() {
            self.parse_as_extern_cmd()
        } else {
            self.parse_as_intern_cmd(cmds)
        }
    }

    fn parse_as_intern_cmd(&self, cmds: Vec<(usize, String)>) -> io::Result<()> {
        for cmd in cmds {
            let (i, cmd) = (cmd.0, Cmds::from(cmd.1));
            let cmd_suffix: String = self
                .buffer
                .split_as_cmd()
                .skip(i + 1)
                .map(|s| s.to_owned() + " ")
                .collect();
            cmd.run(&cmd_suffix);
        }

        Ok(())
    }

    fn parse_as_extern_cmd(&self) -> io::Result<()> {
        crossterm::RawScreen::disable_raw_mode().unwrap();

        let mut items = self.buffer.split_as_cmd();
        let head = match items.next() {
            Some(h) => h,
            None => return Ok(()),
        };
        process::Command::new(head)
            .args(&items.collect::<Vec<String>>())
            .spawn()?
            .wait()?;

        Ok(())
    }
}
