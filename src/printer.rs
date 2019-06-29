use super::utils::StringTools;
use super::PROMPT;
use crossterm::{ClearType, Color};

// write = print without buffer modification
impl super::Rustie {
    pub fn print_hint(&self) {
        if let Some(hint) = self.hints.current() {
            self.cursor.save_position().unwrap();

            let mut hint = hint.file_name().unwrap().to_str().unwrap().to_owned();
            hint.strings_inter(&self.buffer);
            self.print(&hint, Color::Cyan);

            self.cursor.reset_position().unwrap();
        }
    }

    pub fn welcome(&self) {
        let _ = self.terminal.clear(ClearType::All);
        self.print("Welcome to rusite!\n", Color::Blue);
    }

    pub fn print_prompt(&self) {
        self.print(PROMPT, Color::Yellow);
    }

    pub fn print<S: ToString>(&self, s: S, c: Color) {
        let _ = self.color.set_fg(c);
        let _ = self.terminal.write(s.to_string());
        let _ = self.color.reset();
    }
}
