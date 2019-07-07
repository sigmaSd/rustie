pub trait StringTools {
    fn strings_inter(&mut self, s: &str);
    fn split_tokens(&self) -> Vec<String>;
    fn split_cmds(&self) -> Vec<String>;
    fn chars_count(&self) -> usize;
}

impl StringTools for String {
    fn strings_inter(&mut self, s1: &str) {
        let mut idx = self.len();
        loop {
            if !self[..idx].is_empty() && s1.ends_with(&self[..idx]) {
                for _ in 0..idx {
                    self.remove(0);
                }
                break;
            }
            if idx == 0 {
                if let Some(last_char) = s1.chars().last() {
                    if last_char.is_alphanumeric() {
                        self.clear();
                    }
                }
                break;
            }

            idx -= 1;
        }
    }

    fn split_tokens(&self) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        let mut token = String::new();
        let mut quote = false;
        let mut d_quote = false;

        for c in self.chars() {
            match c {
                '"' => d_quote = !d_quote,
                '\'' => quote = !quote,
                ' ' => {
                    if quote || d_quote {
                        token.push(c);
                    } else if !token.is_empty() {
                        tokens.push(token.drain(..).collect());
                    }
                }
                c => token.push(c),
            }
        }

        if !token.is_empty() {
            tokens.push(token.drain(..).collect());
        }
        tokens
    }

    fn split_cmds(&self) -> Vec<String> {
        let mut v = vec![];
        let mut tmp_cmd = String::new();
        let mut quote = false;
        let mut d_quote = false;

        for c in self.chars() {
            match c {
                '\'' => {
                    quote = !quote;
                    tmp_cmd.push(c);
                }
                '"' => {
                    d_quote = !d_quote;
                    tmp_cmd.push(c);
                }
                ';' => {
                    if !quote && !d_quote {
                        v.push(tmp_cmd.drain(..).collect())
                    }
                }
                c => tmp_cmd.push(c),
            }
        }
        v.push(tmp_cmd.drain(..).collect());
        v
    }

    fn chars_count(&self) -> usize {
        self.chars().count()
    }
}

pub fn into_raw_mode() {
    crossterm::RawScreen::into_raw_mode()
        .unwrap()
        .disable_drop();
}

pub fn disable_raw_mode() {
    crossterm::RawScreen::disable_raw_mode().unwrap();
}
