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

fn main() {
    // Languages::Rust ..... operator scope? => ::
    let my_favorite_language = Languages::Rust;

    match my_favorite_language {
        Languages::Typescript => favorite_is_typescript(),
        Languages::Rust => favorite_is_rust(),
        _ => println!("favorite is other"),
    }
}
