use super::utils::StringTools;
use super::{Cmds, Env};
use std::iter;
use std::path;

#[derive(Default, Debug)]
pub struct Hints {
    current_hints: Vec<path::PathBuf>,
    cursor: usize,
}

impl Hints {
    pub fn current(&self) -> Option<&path::PathBuf> {
        self.current_hints.get(self.cursor)
    }

    pub fn cycle(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.current_hints.len() {
            self.cursor = 0;
        }
    }

    pub fn apply_conds(&mut self, c: &[impl Fn(&str) -> bool]) {
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

    fn clear(&mut self) {
        self.current_hints.clear();
        self.cursor = 0;
    }

    pub fn append(&mut self, v: &mut Vec<path::PathBuf>) {
        self.current_hints.append(v);
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

impl super::Rustie {
    pub fn update_hint(&mut self) {
        let tail = self
            .buffer
            .split_as_cmd()
            .last()
            .unwrap_or_else(|| "".to_string());

        if tail.contains('/') {
            let slash_tail = tail.rsplit('/').next().unwrap();
            let mut path = path::Path::new(&tail).components();
            if !tail.ends_with('/') && path.clone().count() > 1 {
                path.next_back();
            }
            let new_env = Env::new(&path);

            // place holder for a correct logic
            self.hints.clear();
            self.hints.append(
                &mut new_env
                    .clone()
                    .into_iter()
                    .filter(|e| {
                        let f_name = e.file_name().unwrap().to_str().unwrap();
                        f_name.starts_with(slash_tail)
                    })
                    .collect(),
            );
        } else if !self.buffer.ends_with(' ') {
            self.hints = self
                .env
                .clone()
                .into_iter()
                .filter(|e| {
                    let f_name = e.file_name().unwrap().to_str().unwrap();
                    f_name.starts_with(&tail)
                })
                .collect();
        } else {
            self.hints = self.env.clone().into_iter().collect();
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
}
