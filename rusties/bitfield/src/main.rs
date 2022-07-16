mod bitfield;
use bitfield::BitField;

const MAX_COMPONENTS: usize = 32;

fn main() {
    let mut my_num = BitField::with_bits(MAX_COMPONENTS);

    my_num.set_nth_bit(0);
    my_num.set_nth_bit(1);
    my_num.set_nth_bit(7);
    my_num.set_nth_bit(30);

    let mut my_num2 = BitField::with_bits(MAX_COMPONENTS);

    my_num2.set_nth_bit(1);
    my_num2.set_nth_bit(7);
    my_num2.set_nth_bit(30);
    
    let n = my_num.and(&my_num2);

    println!("{n}");
}
