fn main() {
    println!("Hello, world!");

    let algo: &str = "algo1";

    match algo {
        "algo" => println!("você disse algo"),
        "outro" => println!("você disse outro"),
        _ => println!("você disse ooops"),
    }
}
