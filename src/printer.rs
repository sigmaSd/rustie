use super::utils::StringTools;
use super::PROMPT;
use crossterm::{ClearType, Color};

// write = print without buffer modification
impl super::Rustie {
    pub fn print_hint(&self) {
        if let Some(hint) = self.hints.current() {
            self.cursor.save_position().unwrap();
            let mut hint = hint.clone();
            hint.strings_inter(&self.buffer);
            self.print(&hint, Color::Cyan);

            self.cursor.reset_position().unwrap();
        }
    }

    pub fn welcome(&mut self) {
        let _ = self.terminal.clear(ClearType::All);
        self.print("Welcome to rusite!", Color::Blue);
        self.new_line();
    }

    pub fn print_prompt(&self) {
        self.print(PROMPT, Color::Yellow);
    }

    pub fn print<S: ToString>(&self, s: S, c: Color) {
        let _ = self.color.set_fg(c);
        let _ = self.terminal.write(s.to_string());
        let _ = self.color.reset();
    }

    pub fn new_line(&mut self) {
        let _ = self.terminal.write("\r\n");
        self.lock_pos.1 += 1;
    }
}

// Overflow handling utilites
impl super::Rustie {
    pub fn screen_height_overflow_by_str(&self, out: &str) -> u16 {
        let screen_size = self.terminal.terminal_size();
        let cursor_pos = self.cursor.pos();
        let new_lines = (out.to_owned().chars_count() as u16 + cursor_pos.0) / screen_size.0;

        (new_lines + cursor_pos.1).saturating_sub(screen_size.1)
    }
}
