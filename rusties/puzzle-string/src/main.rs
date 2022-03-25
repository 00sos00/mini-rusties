#[derive(Debug)]
struct PuzzleString {
    puzzled_string: String,
    unpuzzled_string: String,
}

impl PuzzleString {
    fn from(string: &str) -> Self {
        Self {
            puzzled_string: "_".repeat(string.chars().count()),
            unpuzzled_string: string.to_string(),
        }
    }

    fn puzzled_string(&self) -> String {
        self.puzzled_string.clone()
    }

    fn unpuzzled_string(&self) -> String {
        self.unpuzzled_string.clone()
    }

    fn show_char(&mut self, c: char) {
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

    fn hide_char(&mut self, c: char) {
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

fn main() {
    let mut ps = PuzzleString::from("posse 😀");

    println!(
        "Puzzled: {} - Unpuzzled: {}",
        ps.puzzled_string(),
        ps.unpuzzled_string()
    );

    ps.show_char('o');
    ps.show_char('s');
    ps.show_char('e');

    println!(
        "Puzzled: {} - Unpuzzled: {}",
        ps.puzzled_string(),
        ps.unpuzzled_string()
    );

    ps.hide_char('s');
    ps.hide_char('o');

    println!(
        "Puzzled: {} - Unpuzzled: {}",
        ps.puzzled_string(),
        ps.unpuzzled_string()
    );

    ps.show_char('p');
    ps.show_char('o');
    ps.show_char('s');
    ps.show_char('e');

    println!(
        "Puzzled: {} - Unpuzzled: {}",
        ps.puzzled_string(),
        ps.unpuzzled_string()
    );

    ps.hide_char('e');
    ps.hide_char('o');

    println!(
        "Puzzled: {} - Unpuzzled: {}",
        ps.puzzled_string(),
        ps.unpuzzled_string()
    );

    ps.show_char('😀');

    println!(
        "Puzzled: {} - Unpuzzled: {}",
        ps.puzzled_string(),
        ps.unpuzzled_string()
    );
}
