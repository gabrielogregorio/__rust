struct Npc {
    height_in_cm: u16,
    weight_in_kg: u16,
    name: String,
}

fn main() {
    let junior = Npc {
        height_in_cm: 174,
        weight_in_kg: 87,
        name: String::from("Junior Struct"),
    };

    println!(
        "Npc {} has height {}cm, weight {}kg",
        junior.name, junior.height_in_cm, junior.weight_in_kg
    )
}
