use crossterm::{
    ClearType, Color, Crossterm, InputEvent, KeyEvent, SyncReader, Terminal, TerminalColor,
    TerminalCursor,
};

use std::fs;
use std::io;
use std::path;
use std::process;
mod utils;
use std::iter;
use utils::StringTools;

const PROMPT: &str = "rustie>>> ";

struct Env {
    entrys: Vec<path::PathBuf>,
    cursor: usize,
}

impl Env {
    fn new() -> Self {
        let mut entrys = vec![];
        fs::read_dir("./")
            .unwrap()
            .for_each(|e| entrys.push(e.unwrap().path()));

        Self { entrys, cursor: 0 }
    }

    fn current(&self) -> &path::PathBuf {
        &self.entrys[self.cursor]
    }

    fn cycle(&mut self) {
        self.cursor += 1;
        if self.cursor == self.entrys.len() {
            self.cursor = 0;
        }
    }
}

#[derive(Default, Debug)]
struct Hints {
    current_hints: Vec<String>,
    cursor: usize,
}

impl Hints {
    fn current(&self) -> Option<&String> {
        self.current_hints.get(self.cursor)
    }

    fn cycle(&mut self) {
        self.cursor += 1;
        if self.cursor == self.current_hints.len() {
            self.cursor = 0;
        }
    }

    fn clear(&mut self) {
        self.current_hints.clear();
        self.cursor = 0;
    }
}

impl iter::FromIterator<String> for Hints {
    fn from_iter<I: iter::IntoIterator<Item = String>>(i: I) -> Self {
        let mut current_hints = vec![];
        current_hints.extend(i);

        Self {
            current_hints,
            cursor: 0,
        }
    }
}

struct Rustie {
    input: SyncReader,
    terminal: Terminal,
    color: TerminalColor,
    cursor: TerminalCursor,
    buffer: String,
    hints: Hints,
    env: Env,
    // dont depass this point
    lock_pos: (u16, u16),
}

impl Rustie {
    fn new() -> Self {
        let crossterm = Crossterm::new();
        let input = crossterm.input().read_sync();
        let terminal = crossterm.terminal();
        let color = crossterm.color();
        let cursor = crossterm.cursor();
        let lock_pos = (PROMPT.len() as u16, cursor.pos().1);

        Self {
            input,
            terminal,
            color,
            cursor,
            buffer: String::new(),
            hints: Hints::default(),
            env: Env::new(),
            lock_pos,
        }
    }

    fn print_prompt(&self) {
        let _ = self.color.set_fg(Color::Yellow);
        self.print(PROMPT);
        let _ = self.color.reset();
    }

    fn print<S: ToString>(&self, s: S) {
        let _ = self.terminal.write(s.to_string());
    }

    fn enter(&mut self) {
        self.hints.clear();
        self.print("\n\r");

        let out = self.eval().unwrap_or("".into());
        out.split('\n').for_each(|p| {
            self.print(p);
            self.print("\n\r");
        });

        self.print_prompt();
        self.buffer.clear();
        self.lock_pos.1 = self.cursor.pos().1;
    }

    fn eval(&mut self) -> io::Result<String> {
        let mut items = self.buffer.split_whitespace();
        let out = process::Command::new(items.next().unwrap())
            .args(&items.collect::<Vec<&str>>())
            .output()?;

        let out = if out.stderr.is_empty() {
            out.stdout
        } else {
            out.stderr
        };

        Ok(String::from_utf8(out).unwrap())
    }

    fn tab(&mut self) {
        self.hints.cycle();
        self.cursor.save_position();
        self.color.set_fg(Color::Cyan);
        self.print_hint();

        self.color.reset();
        self.cursor.reset_position();
    }

    fn print_hint(&self) {
        let mut hint = self.hints.current().unwrap_or(&"".to_owned()).to_owned();
        StringTools::strings_unique(&self.buffer, &mut hint);
        self.print(hint);
    }

    fn back_space(&mut self) {
        let cursor_pos = self.cursor.pos();
        if cursor_pos == self.lock_pos {
            return;
        }
        self.buffer.pop();
        self.cursor.move_left(1);
        self.print(" ");
        self.cursor.move_left(1);

        if cursor_pos.0 == 0 {
            self.cursor
                .goto(self.terminal.terminal_size().0, cursor_pos.1 - 1);
        }
    }

    fn update_lock_pos(&mut self) {
        let cursor = self.cursor.pos();
        if (cursor.0 + 1, cursor.1) == self.terminal.terminal_size() {
            self.lock_pos.1 -= 1;
        }
    }

    fn update_tab(&mut self) {
        let last_item = self.buffer.split_whitespace().last().unwrap_or("");

        if self.buffer.chars().last() != Some(' ') {
            self.hints = self
                .env
                .entrys
                .iter()
                .filter_map(|e| {
                    let f_name = e.file_name().unwrap().to_str().unwrap();
                    if f_name.starts_with(last_item) {
                        Some(f_name)
                    } else {
                        None
                    }
                })
                .map(ToOwned::to_owned)
                .collect();
        } else {
            self.hints = self
                .env
                .entrys
                .iter()
                .map(|e| e.file_name().unwrap().to_str().unwrap().to_string())
                .collect();
        }
    }

    fn right(&mut self) {
        if let Some(hint) = self.hints.current() {
            self.print_hint()
        }
    }

    fn run(&mut self) {
        let _screen = crossterm::RawScreen::into_raw_mode().unwrap();

        self.print_prompt();
        self.update_tab();
        loop {
            if let Some(key_ev) = self.input.next() {
                self.terminal.clear(ClearType::UntilNewLine);
                match key_ev {
                    InputEvent::Keyboard(KeyEvent::Char(c)) => match c {
                        '\t' => self.tab(),
                        '\n' => self.enter(),
                        _ => {
                            // order matters
                            self.buffer.push(c);
                            self.update_lock_pos();
                            self.update_tab();
                            self.print(c);
                        }
                    },
                    InputEvent::Keyboard(KeyEvent::Backspace) => {
                        self.back_space();
                    }
                    InputEvent::Keyboard(KeyEvent::Right) => {
                        self.right();
                    }
                    InputEvent::Keyboard(KeyEvent::Ctrl('d')) => {
                        dbg!(self.lock_pos);
                    }
                    InputEvent::Keyboard(KeyEvent::Ctrl('c')) => return (),
                    _ => (),
                }
            }
        }
    }
}

fn main() {
    let mut rustie = Rustie::new();
    rustie.run();
}
