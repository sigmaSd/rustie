use super::utils::StringTools;
use crossterm::Color;
/// writer = print + buffer modification
impl super::Rustie {
    /// use hint, default to current hint
    pub fn use_hint(&mut self, hint: Option<&String>) {
        let mut hint = if let Some(hint) = hint {
            hint.to_string()
        } else if let Some(hint) = self.hints.current() {
            hint.to_string()
        } else {
            return;
        };

        hint.strings_inter(&self.buffer);
        self.print(&hint, Color::DarkYellow);
        self.buffer.push_str(&hint);
    }
}
