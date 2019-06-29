use crossterm::{
    ClearType, Crossterm, InputEvent, KeyEvent, SyncReader, Terminal, TerminalColor, TerminalCursor,
};

mod cmds;
mod utils;
use cmds::Cmds;
mod env;
use env::Env;
mod hints;
use hints::Hints;
mod eval;
mod events;
mod printer;
mod writer;

pub const PROMPT: &str = "rustie>>> ";

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
                        c => self.handle_char(c),
                    },
                    InputEvent::Keyboard(KeyEvent::Backspace) => {
                        self.back_space();
                    }
                    InputEvent::Keyboard(KeyEvent::Right) => {
                        self.right();
                    }
                    InputEvent::Keyboard(KeyEvent::Ctrl('d')) => {
                        self.handle_ctrl_d();
                    }
                    InputEvent::Keyboard(KeyEvent::Ctrl('c')) => {
                        self.handle_ctrl_c();
                    }
                    InputEvent::Keyboard(KeyEvent::Ctrl('l')) => {
                        self.handle_ctrl_l();
                    }
                    InputEvent::Keyboard(KeyEvent::CtrlLeft) => {
                        dbg!(&self.hints);
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
