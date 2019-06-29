pub trait StringTools {
    fn strings_inter(&mut self, s: &str);
    fn split_as_cmd(&self) -> std::vec::IntoIter<String>;
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

    fn split_as_cmd(&self) -> std::vec::IntoIter<String> {
        let mut cmds: Vec<String> = vec![];
        let mut tmp_cmd = String::new();
        let mut quote = false;
        let mut d_quote = false;

        for c in self.chars() {
            match c {
                '"' => d_quote = !d_quote,
                '\'' => quote = !quote,
                ' ' => {
                    if quote || d_quote {
                        tmp_cmd.push(c);
                    } else {
                        cmds.push(tmp_cmd.drain(..).collect());
                    }
                }
                c => tmp_cmd.push(c),
            }
        }

        if !tmp_cmd.is_empty() {
            cmds.push(tmp_cmd.drain(..).collect());
        }
        cmds.into_iter()
    }
}
