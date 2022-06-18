mod my_num;

use my_num::MyNum;

const MAX_COMPONENTS: usize = 16;

// (MAX_COMPONENTS as f32 / 8.0).ceil()
// gives us the minimum amount of bytes needed to store
// [MAX_COMPONENTS] amount of components

fn main() {
    let mut my_num = MyNum::new((MAX_COMPONENTS as f32 / 8.0).ceil() as usize);
    let mut my_num2 = MyNum::new((MAX_COMPONENTS as f32 / 8.0).ceil() as usize);
    my_num.set_nth_bit(1);
    my_num.set_nth_bit(2);
    my_num.set_nth_bit(8);
    my_num2.set_nth_bit(2);
    my_num2.set_nth_bit(8);
    let n = my_num.and(&my_num2);

    println!("{:?}", n.data);
}
