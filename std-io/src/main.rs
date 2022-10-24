use std::{io, io::BufRead}; //FOR SECOND STEP BufRead

fn convert_to_int_and_reference(number: &String) -> i32 {
    return number.trim().parse::<i32>().unwrap();
}

fn convert_to_int(number: String) -> i32 {
    return number.trim().parse::<i32>().unwrap();
}

fn main() {
    let mut number1: String = String::new();
    println!("type a number:");
    io::stdin().read_line(&mut number1).expect("unknow error");

    let mut number2: String = String::new();
    println!("type a number:");
    io::stdin().read_line(&mut number2).expect("unknow error");

    let number_i32: i32 = convert_to_int(number1); // move this variable number to this funcion
                                                   // number1 no available in scope
    if number_i32 > convert_to_int_and_reference(&number2) {
        println!("number1 > number2");
    } else {
        println!("number1 <= number2");
    }

    println!("number1 {}", number_i32);
    println!("number2 {}", number2);

    // SECOND STEP

    let stdin = io::stdin();
    let mut text: String = String::new();

    println!("here text:");
    text = stdin.lock().lines().next().unwrap().unwrap();

    println!("{}", text);
}
