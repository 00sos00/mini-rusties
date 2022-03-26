#[derive(Debug)]
pub struct PuzzleString {
    puzzled_string: String,
    unpuzzled_string: String,
}

impl PuzzleString {
    pub fn from(string: &str) -> Self {
        Self {
            puzzled_string: "_".repeat(string.chars().count()),
            unpuzzled_string: string.to_string(),
        }
    }

    pub fn puzzled_string(&self) -> String {
        self.puzzled_string.clone()
    }

    pub fn unpuzzled_string(&self) -> String {
        self.unpuzzled_string.clone()
    }

    pub fn show_char(&mut self, c: char) {
        assert!(
            self.unpuzzled_string.contains(c),
            "'{c}' Character not found in string"
        );

        self.puzzled_string = self
            .unpuzzled_string
            .chars()
            .zip(self.puzzled_string.chars())
            .map(|(u, p)| if u == c { u } else { p })
            .collect();
    }

    pub fn hide_char(&mut self, c: char) {
        assert!(
            self.unpuzzled_string.contains(c),
            "'{c}' Character not found in string"
        );

        self.puzzled_string = self
            .puzzled_string
            .chars()
            .map(|p| if p == c { '_' } else { p })
            .collect();
    }
}