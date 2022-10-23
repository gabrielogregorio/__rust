fn main() {
    let vector = vec![10, 20, 30, 40, 50];
    println!("{}", vector[1]);

    let mut vector2: Vec<u8> = Vec::new();
    vector2.push(1);
    vector2.push(122);
    vector2.push(2);
    println!("{}, {:?}", vector2[1], vector2);
    vector2.remove(2);
    println!("{}, {:?}", vector2[1], vector2);
}
