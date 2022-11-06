// this enum exists by default
//// enum Option<T> {
////   Some(T), exists value
////   None, value is None
//// }

enum Languages {
    Rust,
    Typescript,
    Python,
    CSharp,
}

fn favorite_is_typescript() {
    println!("favorite is typescript");
}

fn favorite_is_rust() {
    println!("favorite is rust");
}

enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Languages::Rust ..... operator scope? => ::
    let my_favorite_language = Languages::Rust;

    match my_favorite_language {
        Languages::Typescript => favorite_is_typescript(),
        Languages::Rust => favorite_is_rust(),
        _ => println!("favorite is other"),
    }

    IpAddress::V4(127, 0, 0, 1);
    IpAddress::V6(String::from("::"));

    // different types=>

    let x: i8 = 10;
    let y: Option<i8> = Some(20);

    if y.is_some() {
        let sum = x + y.unwrap();
        println!("{}", sum);
    }
}
