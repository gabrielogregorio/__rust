fn test(item: String) -> (usize, String) {
    (item.len(), item)
}

fn test_with_reference(item: &mut String) -> usize {
    item.push_str("PUSHED");
    item.len()
}

fn main() {
    let a: i8 = 2;
    let b_clone_value: i8 = a.clone();

    println!("value: {}", a); // 2
    println!("reference or pointer: {:p}", &a); //  0x7fff165aab0f
    println!("reference or pointer: {:p}", &b_clone_value); //  0x7ffce03cf207

    // move and return value
    let item = String::from("10");
    let (result, item) = test(item);

    let mut item2 = String::from("10");
    let result2 = test_with_reference(&mut item2);

    println!("here {} {} {} {}", item, item2, result, result2);
}
