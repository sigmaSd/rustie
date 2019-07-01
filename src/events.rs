use crossterm::Color;

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
        self.update_lock_pos();
        // update env if a new slash is added or deleted (In/Out dir)
        if c == '/' {
            self.env.update(&self.buffer);
        }
        self.update_hint();
        self.print(c, Color::DarkYellow);
        self.print_hint();
    }

    pub fn enter(&mut self) {
        self.print("\n\r", Color::White);
        let _ = self.eval();
        self.print_prompt();
        self.buffer.clear();
        self.env.reset();
        self.update_hint();
        self.lock_pos.1 = self.cursor.pos().1;
    }

    pub fn handle_ctrl_c(&mut self) {
        self.buffer.clear();
        self.print("\r\n", Color::White);
        self.print_prompt();
    }

    pub fn handle_ctrl_d(&mut self) {
        if self.buffer.is_empty() {
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
}

/// keep the lock in sync
impl super::Rustie {
    fn update_lock_pos(&mut self) {
        let cursor = self.cursor.pos();
        if (cursor.0 + 1, cursor.1) == self.terminal.terminal_size() {
            self.lock_pos.1 -= 1;
        }
    }
}
