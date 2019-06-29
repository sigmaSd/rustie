use std::fs;
use std::path;

#[derive(Default, Clone)]
pub struct Env {
    entrys: Vec<path::PathBuf>,
    _cursor: usize,
}

impl Env {
    pub fn new<P: AsRef<path::Path>>(p: P) -> Self {
        let mut entrys = vec![];

        if let Ok(f) = fs::read_dir(p.as_ref()) {
            f.for_each(|e| entrys.push(e.unwrap().path()));
            Self { entrys, _cursor: 0 }
        } else {
            Self::default()
        }
    }

    pub fn update(&mut self) {
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

impl IntoIterator for Env {
    type Item = path::PathBuf;
    type IntoIter = ::std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.entrys.into_iter()
    }
}
