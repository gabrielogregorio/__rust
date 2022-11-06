#[derive(Debug)]

struct Npc {
    height_in_cm: u16,
    weight_in_kg: u16,
    name: String,
}

#[derive(Debug)]
struct Pointer(i8, i8, i8);

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

    let pointer = Pointer(1, 2, 6);
    println!("POINTER x={}, y={}. z={}", pointer.0, pointer.1, pointer.2);

    println!("POINTER {:#?}", pointer);
}
