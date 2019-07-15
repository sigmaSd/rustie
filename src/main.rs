use crossterm::{
    ClearType, Crossterm, InputEvent, KeyEvent, SyncReader, Terminal, TerminalColor, TerminalCursor,
};

mod cmds;
mod utils;
use cmds::Cmds;
mod paths;
use paths::Paths;
mod hints;
use hints::Hints;
mod history;
use history::History;
mod envs;
use envs::Envs;
mod bins;
use bins::Bins;
mod config;
use config::Config;
mod eval;
mod events;
mod printer;
mod writer;

pub const PROMPT: &str = "rustie";

struct Rustie {
    input: SyncReader,
    terminal: Terminal,
    color: TerminalColor,
    cursor: TerminalCursor,
    buffer: String,
    hints: Hints,
    paths: Paths,
    envs: Envs,
    // dont depass this point
    lock_pos: (u16, u16),
    history: History,
    bins: Bins,
}

impl Rustie {
    fn new() -> Self {
        let crossterm = Crossterm::new();
        let input = crossterm.input().read_sync();
        let terminal = crossterm.terminal();
        let color = crossterm.color();
        let cursor = crossterm.cursor();
        let lock_pos = (PROMPT.len() as u16, 0);
        let history = History::new(dirs::cache_dir().unwrap().join("rustie")).unwrap_or_default();

        let mut envs = Envs::new();
        Config::parse_config(&mut envs);
        envs.update_os_path();

        let bins = Bins::new(&envs);

        Self {
            input,
            terminal,
            color,
            cursor,
            buffer: String::new(),
            hints: Hints::default(),
            paths: Paths::new("./"),
            envs,
            bins,
            lock_pos,
            history,
        }
    }

    fn run(&mut self) {
        self.welcome();
        self.print_prompt();
        self.update_hint();
        utils::into_raw_mode();
        loop {
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
                    InputEvent::Keyboard(KeyEvent::Up) => {
                        self.up();
                    }
                    InputEvent::Keyboard(KeyEvent::Down) => {
                        self.down();
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

fn disable_ctrl_c() {
    let _ = ctrlc::set_handler(|| {});
}

fn main() {
    disable_ctrl_c();
    let mut rustie = Rustie::new();
    rustie.run();
}
