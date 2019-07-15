use std::fs;
use std::io;
use std::path;

#[derive(Default)]
pub struct History {
    history: Vec<String>,
    current: String,
    cursor: usize,
    path: path::PathBuf,
}

impl History {
    pub fn new(path: path::PathBuf) -> io::Result<Self> {
        let _ = fs::create_dir_all(&path);

        let path = path.join("history");
        if !path.exists() {
            let _ = fs::File::create(&path);
        }

        let history: Vec<String> = fs::read_to_string(&path)?
            .lines()
            .map(ToOwned::to_owned)
            .collect();
        let cursor = history.len();
        let current = String::new();

        Ok(Self {
            history,
            current,
            cursor,
            path,
        })
    }
    pub fn down(&mut self) -> Option<String> {
        let filtered = self.filter();
        self.cursor += 1;
        if self.cursor >= filtered.len() {
            self.cursor = filtered.len();
            Some(self.current.clone())
        } else {
            Some(filtered[self.cursor].clone())
        }
    }

    pub fn up(&mut self) -> Option<String> {
        let filtered = self.filter();
        self.cursor = std::cmp::min(self.cursor, filtered.len());
        if self.cursor == 0 || filtered.is_empty() {
            None
        } else {
            self.cursor = self.cursor.saturating_sub(1);
            Some(filtered[self.cursor].clone())
        }
    }

    pub fn push(&mut self, buffer: String) {
        if !buffer.is_empty() && Some(&buffer) != self.history.last() {
            self.current.clear();
            self.history.push(buffer);
            self.go_to_last();
        }
    }

    pub fn update_current(&mut self, buffer: &str) {
        self.current = buffer.to_string();
        self.cursor = self.history.len();
    }

    pub fn save(&mut self) {
        self.keep_unique_history();
        let history: String = self.history.join("\n");
        let _ = fs::write(&self.path, history);
    }

    fn keep_unique_history(&mut self) {
        // remove all duplicates while keeping the latest one
        let mut i: i32 = self.history.len() as i32 - 1;
        while i >= 0 {
            let current_his = self.history[i as usize].clone();
            let mut j = 1;
            let mut removed = 0;
            while i - j >= 0 {
                let search_idx = (i - j) as usize;
                if current_his.trim() == self.history[search_idx].trim() {
                    self.history.remove(search_idx);
                    removed += 1;
                }
                j += 1;
            }
            i -= 1 + removed;
        }
    }

    pub fn get(&self) -> &[String] {
        &self.history
    }

    fn filter(&self) -> Vec<String> {
        self.history
            .iter()
            .filter(|h| h.contains(&self.current))
            .map(ToOwned::to_owned)
            .collect()
    }

    fn go_to_last(&mut self) {
        if !self.history.is_empty() {
            self.cursor = self.history.len();
        }
    }
}
