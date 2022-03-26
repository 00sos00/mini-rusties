mod puzzle_string;

use puzzle_string::PuzzleString;

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
