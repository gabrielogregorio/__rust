use std::collections::HashMap;

fn main() {
    let mut movies: HashMap<&str, i8> = HashMap::new();

    movies.insert("poder sem limites", 9);
    movies.insert("Logan", 10);
    movies.insert("Os Incríveis 1", 10);
    movies.insert("Os Incríveis 2", 10);
    movies.insert("Cavaleiro da Lua", 10);
    movies.insert("Pearl Harbor", 9);
    movies.insert("Parasita", 11);
    let return_value_or_insert_not_replace = movies.entry("Parasita").or_insert(10);
    println!("RETURNED {}", return_value_or_insert_not_replace);

    movies.insert("Oblivion", 9);
    movies.insert("Perdido em Marte", 10);

    println!("Hello, world! {:?}", movies);
    println!("item {:?}", movies.get("Parasita")); // Some or None
    println!("item {:?}", movies.get("Unknown")); // Some or None

    for (key, value) in &movies {
        println!("{}...{}", key, value);
    }
}
