use std::{ io::stdin, io::BufRead};
use rand::Rng;
use std::cmp::Ordering;


fn sort_number() -> String{
  rand::thread_rng().gen_range(1..5).to_string()
}

fn main() {    
    println!("Digite um número entre 1 e 4"); // bug from 5?
    let guess:String =  stdin().lock().lines().next().unwrap().unwrap();
    
    let sorted_number:String = sort_number();

    match guess.cmp(&sorted_number) {
        Ordering::Less => println!("higher please! => {}", sorted_number),
        Ordering::Greater => println!("lower please! => {}", sorted_number),
        Ordering::Equal => println!("NICE!!!"),
    }
}
