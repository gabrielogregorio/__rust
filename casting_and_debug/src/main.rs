fn magic(n1: f32, n2: u8) -> f32 {
    return n1 + n2 as f32;
}

fn main() {
    let number: i32 = 1123;

    println!("number {}", number);
    println!("Hello, world! {}", magic(10f32, 20u8));
    println!("Hello, world! {}", magic(14f32, 32u8));

    let input: String = String::from("12a");

    let number: i32 = input.parse().expect("Not is number");
    println!("{}", number);
}
