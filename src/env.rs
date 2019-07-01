use super::utils::StringTools;
use std::fs;
use std::path;

#[derive(Clone)]
pub struct Env {
    entrys: Vec<path::PathBuf>,
    origin: path::PathBuf,
    _cursor: usize,
}

impl Default for Env {
    fn default() -> Self {
        Self {
            entrys: vec![],
            origin: path::Path::new("./").to_path_buf(),
            _cursor: 0,
        }
    }
}

impl Env {
    pub fn new<P: AsRef<path::Path>>(p: P) -> Self {
        let entrys = Self::read_path(p.as_ref());

        Self {
            entrys,
            origin: p.as_ref().to_path_buf(),
            _cursor: 0,
        }
    }

    pub fn update(&mut self, buffer: &str) {
        let tail = buffer
            .to_owned()
            .split_as_cmd()
            .last()
            .unwrap_or_else(|| "".to_string());

        let mut path = path::Path::new(&tail).components();
        if !tail.ends_with('/') && path.clone().count() > 1 {
            path.next_back();
        }

        self.entrys.clear();
        self.entrys.append(&mut Self::read_path(&path));
    }

    pub fn reset(&mut self) {
        self.entrys.clear();
        self.entrys.append(&mut Self::read_path(&self.origin));
    }

    fn read_path<P: AsRef<path::Path>>(path: P) -> Vec<path::PathBuf> {
        if let Ok(f) = fs::read_dir(path.as_ref()) {
            f.map(|e| e.unwrap().path()).collect()
        } else {
            vec![]
        }
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

impl IntoIterator for Env {
    type Item = path::PathBuf;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entrys.into_iter()
    }
}
