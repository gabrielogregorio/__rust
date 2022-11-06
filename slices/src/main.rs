fn find_first_world(phrase: &String) -> usize {
    let bytes = phrase.as_bytes();

    for (index, &character) in bytes.iter().enumerate() {
        if (character == b' ') {
            return index;
        }
    }

    return bytes.len();
}

fn get_first_world(phrase: &String) -> &str {
    let bytes = phrase.as_bytes();

    for (index, &character) in bytes.iter().enumerate() {
        if (character == b' ') {
            return &phrase[0..index];
        }
    }

    return &phrase[0..bytes.len()];
}

fn main() {
    println!("Hello, world!");
    let mut phrase = String::from("large string");

    let first_world = find_first_world(&phrase);
    phrase.clear();

    println!(
        "{} - usize is invalid, position not exists! {}",
        phrase, first_world
    );

    // ------------------------

    let mut phrase2 = String::from("large string");

    let first_world2 = get_first_world(&phrase2);
    // phrase2.clear(); // error detected here!
    println!("{} - is valid! {}", phrase2, first_world2);
}
