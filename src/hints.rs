use super::utils::StringTools;
use super::Cmds;
use std::iter;

#[derive(Default, Debug)]
pub struct Hints {
    current_hints: Vec<String>,
    cursor: usize,
}

impl Hints {
    pub fn current(&self) -> Option<&String> {
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
                    if !cc(i) {
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

    pub fn append(&mut self, v: &mut Vec<String>) {
        self.current_hints.append(v);
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

impl super::Rustie {
    pub fn update_hint(&mut self) {
        if self.buffer.is_empty() {
            return;
        }

        let tail = self
            .buffer
            .split_tokens()
            .last()
            .cloned()
            .unwrap_or_default();
        self.hints.clear();

        // add hitory hints
        self.hints.append(
            &mut self
                .history
                .get()
                .iter()
                .filter(|h| h.starts_with(&self.buffer))
                .rev()
                .cloned()
                .collect(),
        );

        // add path hints
        if !self.buffer.ends_with(' ') {
            self.hints.append(
                &mut self
                    .paths
                    .clone()
                    .into_iter()
                    .filter(|e| {
                        let f_name = e.file_name().unwrap().to_str().unwrap();
                        if tail.contains('/') {
                            let slash_tail = tail.rsplit('/').next().unwrap();
                            f_name.starts_with(&slash_tail)
                        } else {
                            f_name.starts_with(&tail)
                        }
                    })
                    .map(|e| e.to_str().unwrap().trim_start_matches("./").to_string())
                    .collect(),
            );
        } else {
            self.hints.append(
                &mut self
                    .paths
                    .clone()
                    .into_iter()
                    .map(|e| e.to_str().unwrap().trim_start_matches("./").to_string())
                    .collect(),
            );
        }

        // add var hints
        self.hints.append(
            &mut self
                .envs
                .keys()
                .into_iter()
                .filter(|s| s.starts_with(&tail.trim_start_matches('$')))
                .collect(),
        );

        // add bins hints
        self.hints.append(
            &mut self
                .bins
                .keys()
                .into_iter()
                .filter(|s| s.starts_with(&tail))
                .collect(),
        );

        self.check_cmd_hint();
    }

    fn check_cmd_hint(&mut self) {
        let cmds = self.buffer.split_cmds();
        let mut hint_conditions = vec![];
        for cmd in cmds {
            if let Some(cmd) = &cmd.split_tokens().get(0) {
                if Cmds::contains(&cmd) {
                    let cmd = Cmds::from(cmd.as_str());
                    hint_conditions.push(cmd.get_hint_cond());
                }
            }
        }

        self.hints.apply_conds(&hint_conditions);
    }
}
