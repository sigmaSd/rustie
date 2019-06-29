use super::utils::StringTools;
use super::PROMPT;
use crossterm::Color;

// write = print without buffer modification
impl super::Rustie {
    pub fn print_hint(&self) {
        if let Some(hint) = self.hints.current() {
            self.cursor.save_position().unwrap();
            self.color.set_fg(Color::Cyan).unwrap();

            let mut hint = hint.file_name().unwrap().to_str().unwrap().to_owned();
            hint.strings_inter(&self.buffer);
            self.print(&hint);

            self.color.reset().unwrap();
            self.cursor.reset_position().unwrap();
        }
    }

    pub fn print_prompt(&self) {
        let _ = self.color.set_fg(Color::Yellow);
        self.print(PROMPT);
        let _ = self.color.reset();
    }

    pub fn print<S: ToString>(&self, s: S) {
        let _ = self.terminal.write(s.to_string());
    }
}
