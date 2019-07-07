use super::utils::StringTools;
use crossterm::Color;
/// writer = print + buffer modification
impl super::Rustie {
    pub fn use_hint(&mut self) {
        if let Some(hint) = self.hints.current() {
            let mut hint = hint.clone();
            hint.strings_inter(&self.buffer);
            self.print(&hint, Color::DarkYellow);
            self.buffer.push_str(&hint);
        }
    }
}
