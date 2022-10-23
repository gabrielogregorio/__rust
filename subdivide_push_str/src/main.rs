
fn calculate(number: u8) -> String {

    // new string... rs
    let mut options:String = String::new();

    if number % 3 == 0{
        options.push_str("divisible_by_3");
    }

    if number % 2 == 0 {
        options.push_str("divisible_by_2")
    }

    if options.is_empty() {
        return "not disisible".to_string();
    }

    return  options;

}

fn main() {
    let result:String = calculate(7);
    println!("{}", result);
}
