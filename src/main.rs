use crossterm::{
    ClearType, Color, Crossterm, InputEvent, KeyEvent, SyncReader, Terminal, TerminalColor,
    TerminalCursor,
};

use std::fs;
use std::io;
use std::path;
use std::process;
mod utils;
use std::env;
use std::iter;
use utils::StringTools;

const PROMPT: &str = "rustie>>> ";

enum Cmds {
    Cd,
    Exit,
}

impl ToString for Cmds {
    fn to_string(&self) -> String {
        use Cmds::*;
        match self {
            Cd => "cd".into(),
            Exit => "exit".into(),
        }
    }
}

impl From<String> for Cmds {
    fn from(s: String) -> Cmds {
        use Cmds::*;
        match s.as_str() {
            "cd" => Cd,
            "exit" => Exit,
            _ => unimplemented!(),
        }
    }
}

impl Cmds {
    fn extract_cmds(s: &str) -> Vec<(usize, String)> {
        let cmds = ["cd", "exit"];
        let mut extracted_cmds = vec![];
        for c in cmds.iter() {
            if let Some(n) = s.find(c) {
                extracted_cmds.push((n, c.to_string()))
            }
        }

        extracted_cmds
    }

    fn get_hint_cond(&self) -> Box<Fn(&str) -> bool> {
        use Cmds::*;
        match self {
            Cd => Box::new(|i: &str| path::Path::new(i).is_dir()),
            Exit => Box::new(|_: &str| false),
        }
    }

    fn run(&self, s: &str) {
        use Cmds::*;
        match self {
            Cd => env::set_current_dir(s.split_whitespace().next().unwrap()).unwrap(),
            Exit => {
                if s.is_empty() {
                    process::exit(0)
                }
            }
        }
    }
}

#[derive(Default)]
struct Env {
    entrys: Vec<path::PathBuf>,
    _cursor: usize,
}

impl Env {
    fn new<P: AsRef<path::Path>>(p: P) -> Self {
        let mut entrys = vec![];

        if let Ok(f) = fs::read_dir(p.as_ref()) {
            f.for_each(|e| entrys.push(e.unwrap().path()));
            Self { entrys, _cursor: 0 }
        } else {
            Self::default()
        }
    }

    fn update(&mut self) {
        *self = Self::new("./");
    }

    fn _current(&self) -> &path::PathBuf {
        &self.entrys[self._cursor]
    }

    fn _cycle(&mut self) {
        self._cursor += 1;
        if self._cursor >= self.entrys.len() {
            self._cursor = 0;
        }
    }
}

#[derive(Default, Debug)]
struct Hints {
    current_hints: Vec<path::PathBuf>,
    cursor: usize,
}

impl Hints {
    fn current(&self) -> Option<&path::PathBuf> {
        self.current_hints.get(self.cursor)
    }

    fn cycle(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.current_hints.len() {
            self.cursor = 0;
        }
    }

    fn _clear(&mut self) {
        self.current_hints.clear();
        self.cursor = 0;
    }

    fn apply_conds(&mut self, c: &[impl Fn(&str) -> bool]) {
        self.current_hints = self
            .current_hints
            .drain(..)
            .filter(|i| {
                for cc in c {
                    if !cc(i.to_str().unwrap()) {
                        return false;
                    }
                }
                true
            })
            .collect();
    }
}

impl iter::FromIterator<path::PathBuf> for Hints {
    fn from_iter<I: iter::IntoIterator<Item = path::PathBuf>>(i: I) -> Self {
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
            env: Env::new("./"),
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
        self.print("\n\r");
        let _ = self.eval();
        self.print_prompt();
        self.buffer.clear();
        self.env.update();
        self.update_hint();
        self.lock_pos.1 = self.cursor.pos().1;
    }

    fn eval(&mut self) -> io::Result<()> {
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
                .split_whitespace()
                .skip(i + 1)
                .map(|s| s.to_owned() + " ")
                .collect();
            cmd.run(&cmd_suffix);
        }

        Ok(())
    }

    fn parse_as_extern_cmd(&self) -> io::Result<()> {
        crossterm::RawScreen::disable_raw_mode().unwrap();

        let mut items = self.buffer.split_whitespace();
        let head = match items.next() {
            Some(h) => h,
            None => return Ok(()),
        };
        process::Command::new(head)
            .args(&items.collect::<Vec<&str>>())
            .spawn()?
            .wait()?;

        Ok(())
    }

    fn tab(&mut self) {
        self.hints.cycle();
        self.print_hint();
    }

    fn print_hint(&self) {
        if let Some(hint) = self.hints.current() {
            self.cursor.save_position().unwrap();
            self.color.set_fg(Color::Cyan).unwrap();

            let mut hint = hint.file_name().unwrap().to_str().unwrap().to_owned();
            StringTools::strings_unique(&self.buffer, &mut hint);
            self.print(&hint);

            self.color.reset().unwrap();
            self.cursor.reset_position().unwrap();
        }
    }

    fn use_hint(&mut self) {
        if let Some(hint) = self.hints.current() {
            let mut hint = hint.file_name().unwrap().to_str().unwrap().to_owned();
            StringTools::strings_unique(&self.buffer, &mut hint);
            self.print(&hint);
            self.buffer.push_str(&hint);
        }
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
        self.update_hint();

        if cursor_pos.0 == 0 {
            self.cursor
                .goto(self.terminal.terminal_size().0, cursor_pos.1 - 1)
                .unwrap();
        }
    }

    fn update_lock_pos(&mut self) {
        let cursor = self.cursor.pos();
        if (cursor.0 + 1, cursor.1) == self.terminal.terminal_size() {
            self.lock_pos.1 -= 1;
        }
    }

    fn update_hint(&mut self) {
        let tail = self.buffer.split_whitespace().last().unwrap_or("");

        if tail.contains('/') {
            let slash_tail = tail.rsplit('/').next().unwrap();
            let mut path = path::Path::new(tail).components();
            if !tail.ends_with('/') && path.clone().count() > 1 {
                path.next_back();
            }
            let new_env = Env::new(&path);

            self.hints.current_hints = new_env
                .entrys
                .into_iter()
                .filter(|e| {
                    let f_name = e.file_name().unwrap().to_str().unwrap();
                    f_name.starts_with(slash_tail)
                })
                .collect();
        } else if !self.buffer.ends_with(' ') {
            self.hints = self
                .env
                .entrys
                .iter()
                .filter(|e| {
                    let f_name = e.file_name().unwrap().to_str().unwrap();
                    f_name.starts_with(tail)
                })
                .map(ToOwned::to_owned)
                .collect();
        } else {
            self.hints = self.env.entrys.iter().map(ToOwned::to_owned).collect();
        }

        self.check_cmd_hint();
    }

    fn check_cmd_hint(&mut self) {
        let cmds = Cmds::extract_cmds(&self.buffer);
        let mut hint_conditions = vec![];
        for cmd in cmds {
            let (_i, cmd) = (cmd.0, Cmds::from(cmd.1));
            hint_conditions.push(cmd.get_hint_cond());
        }

        // apply cond
        self.hints.apply_conds(&hint_conditions);
    }

    fn right(&mut self) {
        if self.hints.current().is_some() {
            self.use_hint()
        }
    }

    fn run(&mut self) {
        self.print_prompt();
        self.update_hint();
        loop {
            crossterm::RawScreen::into_raw_mode()
                .unwrap()
                .disable_drop();
            if let Some(key_ev) = self.input.next() {
                self.terminal.clear(ClearType::UntilNewLine).unwrap();
                match key_ev {
                    InputEvent::Keyboard(KeyEvent::Char(c)) => match c {
                        '\t' => self.tab(),
                        '\n' => self.enter(),
                        _ => {
                            // order matters
                            self.buffer.push(c);
                            self.update_lock_pos();
                            self.update_hint();
                            self.print(c);
                            self.print_hint();
                        }
                    },
                    InputEvent::Keyboard(KeyEvent::Backspace) => {
                        self.back_space();
                    }
                    InputEvent::Keyboard(KeyEvent::Right) => {
                        self.right();
                    }
                    InputEvent::Keyboard(KeyEvent::Ctrl('d')) => {
                        //dbg!(&self.hints.current_hints);
                    }
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
