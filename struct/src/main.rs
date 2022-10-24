#[derive(Debug)]

struct Npc {
    height_in_cm: u16,
    weight_in_kg: u16,
    name: String,
}

fn main() {
    let mut junior = Npc {
        height_in_cm: 174,
        weight_in_kg: 87,
        name: String::from("Junior Struct"),
    };

    let programer = Npc {
        name: String::from("Programmer"),
        ..junior
    };

    println!(
        "Npc {} has height {}cm, weight {}kg",
        junior.name, junior.height_in_cm, junior.weight_in_kg
    );

    println!("Npc {:?}", junior);
    println!("Npc {:?}", programer);

    println!("NPC NAME {}", junior.name);
    junior.name = String::from("new name");
    println!("NPC NAME {}", junior.name);
}
