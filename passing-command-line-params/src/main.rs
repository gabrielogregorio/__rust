use std::env;

fn main() {
    // ::args return iterator
    // go collect iterator as vector string
    let args: Vec<String> = env::args().collect();

    // or
    // use turbofish
    let args2: Vec<String> = env::args().collect::<Vec<String>>();

    if args2[0] == "./main" {
        println!("Has ./main");
    }

    println!("Hello, world! {:?} {:?}", args, args2);
}
