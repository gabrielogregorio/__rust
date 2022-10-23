use std::io;

fn convert_to_int(number: &String) -> i32 {
    return number.trim().parse::<i32>().unwrap();
}

fn main() {
    let mut number1: String = String::new();
    println!("type a number:");
    io::stdin().read_line(&mut number1).expect("unknow error");

    let mut number2: String = String::new();
    println!("type a number:");
    io::stdin().read_line(&mut number2).expect("unknow error");

    if convert_to_int(&number1) > convert_to_int(&number2) {
        println!("number1 > number2");
    } else {
        println!("number1 <= number2");
    }
}
