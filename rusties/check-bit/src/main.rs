fn check_bit(index: usize, binary: usize) -> bool {
    (binary & (1 << (index - 1))) > 0
}

fn main() {
    // Index starts at 1
    println!("{}", check_bit(1, 0b00000001));
}
