pub struct StringTools {}

impl StringTools {
    pub fn strings_unique(s1: &str, s2: &mut String) {
        let mut idx = s2.len();
        loop {
            if !s2[..idx].is_empty() && s1.ends_with(&s2[..idx]) {
                for _ in 0..idx {
                    s2.remove(0);
                }
                break;
            }
            if idx == 0 {
                if let Some(last_char) = s1.chars().last() {
                    if last_char.is_alphanumeric() {
                        s2.clear();
                    }
                }
                break;
            }

            idx -= 1;
        }
    }
}
