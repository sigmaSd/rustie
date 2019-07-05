use super::utils;
use crossterm::{ClearType, Color};

impl super::Rustie {
    pub fn right(&mut self) {
        if self.hints.current().is_some() {
            self.use_hint()
        }
    }

    pub fn tab(&mut self) {
        self.hints.cycle();
        self.print_hint();
    }

    pub fn back_space(&mut self) {
        let cursor_pos = self.cursor.pos();
        if cursor_pos == self.lock_pos {
            return;
        }
        self.buffer.pop();
        self.history.update_current(&self.buffer);
        self.cursor.move_left(1);
        self.print(" ", Color::White);
        self.cursor.move_left(1);
        self.update_hint();

        if cursor_pos.0 == 0 {
            self.cursor
                .goto(self.terminal.terminal_size().0, cursor_pos.1 - 1)
                .unwrap();
        }
    }

    pub fn handle_char(&mut self, c: char) {
        // order matters
        self.buffer.push(c);
        self.history.update_current(&self.buffer);
        // update env if a new slash is added or deleted (In/Out dir)
        if c == '/' {
            self.env.update(&self.buffer);
        }
        self.update_hint();
        self.print(c, Color::DarkYellow);
        self.print_hint();
        self.update_lock_pos_with_scroll();
    }

    pub fn enter(&mut self) {
        self.new_line();

        let _ = self.eval();
        utils::into_raw_mode();
        self.sync_lock();
        self.print("\r", crossterm::Color::White);

        self.print_prompt();
        self.history.push(self.buffer.drain(..).collect());
        self.env.reset();
        self.update_hint();
    }

    pub fn handle_ctrl_c(&mut self) {
        self.buffer.clear();
        self.new_line();
        self.print_prompt();
        self.update_lock_pos_with_scroll();
    }

    pub fn handle_ctrl_d(&mut self) {
        if self.buffer.is_empty() {
            self.history.save();
            let _ = self.terminal.clear(crossterm::ClearType::All);
            self.terminal.exit();
        }
    }

    pub fn handle_ctrl_l(&mut self) {
        let _ = self.terminal.clear(crossterm::ClearType::All);
        self.print_prompt();
        self.lock_pos = (super::PROMPT.len() as u16, 0);
        self.print(&self.buffer, Color::DarkYellow);
    }

    pub fn up(&mut self) {
        let up = self.history.up();
        if let Some(up) = up {
            self.buffer = up.clone();
            self.lock_pos.1 -= self.screen_height_overflow_by_str(&up);
            let _ = self
                .cursor
                .goto(self.lock_pos.0 as u16, self.lock_pos.1 as u16);
            let _ = self.terminal.clear(ClearType::FromCursorDown);
            self.print(&up, Color::DarkYellow);
        }
    }

    pub fn down(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let down = self.history.down();
        if let Some(down) = down {
            self.buffer = down.clone();

            self.lock_pos.1 -= self.screen_height_overflow_by_str(&down);
            let _ = self
                .cursor
                .goto(self.lock_pos.0 as u16, self.lock_pos.1 as u16);
            let _ = self.terminal.clear(ClearType::FromCursorDown);
            self.print(&down, Color::DarkYellow);
        }
    }
}

/// keep the lock in sync
impl super::Rustie {
    fn update_lock_pos_with_scroll(&mut self) {
        let cursor = self.cursor.pos();
        if (cursor.0 + 1, cursor.1) == self.terminal.terminal_size() {
            self.lock_pos.1 -= 1;
        }
    }
    fn sync_lock(&mut self) {
        self.lock_pos.1 = self.cursor.pos().1;
    }
}
