fn main() {
    let mut long_tuple: (u8, i32, f32, char, bool, (i8, i8)) =
        (1u8, -100i32, 982f32, 'b', false, (1, 2));

    println!("Tuple {:?} {}", long_tuple, (long_tuple.5).1);
    long_tuple.5 .1 = 99;
    println!("Tuple {:?} {}", long_tuple, (long_tuple.5).1);
}
