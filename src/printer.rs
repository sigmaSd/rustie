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

    pub fn print_prompt(&mut self) {
        let mut cwd = std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        cwd = cwd.replace(dirs::home_dir().unwrap().to_str().unwrap(), "~");

        let mut cwd: Vec<&str> = cwd.split('/').collect();
        let tail = cwd.pop().unwrap();

        let cwd: Vec<String> = cwd
            .into_iter()
            .map(|s| s.chars().nth(0).unwrap_or_default().to_string())
            .collect();
        let cwd = cwd.join("/");

        let path = if cwd.is_empty() {
            tail.to_string()
        } else {
            cwd + "/" + tail
        };

        self.print(PROMPT, Color::Yellow);
        self.print(&path, Color::Green);
        self.print("> ", Color::White);

        let x = PROMPT.len() + path.len() + 2;
        self.lock_pos.0 = x as u16;
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
