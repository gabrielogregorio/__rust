fn magic_example(input: ( f64, bool)) -> (bool, u32, f64) {
    let (any_64, any_bool) = input;
    return (any_bool, 10u32, any_64);
}

fn main() {
    println!("hey {:?}",  magic_example((10.0, true)));
}
