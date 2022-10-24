fn main() {
    let vector = vec![10, 20, 30, 40, 50];
    println!("{}", vector[1]);

    let mut vector2: Vec<u8> = Vec::new();
    vector2.push(1);
    vector2.push(122);
    vector2.push(2);
    vector2.push(3);
    vector2.pop();
    println!("{}, {:?}", vector2[1], vector2);
    vector2.remove(2);
    println!("{:?}", &vector2[0..2]);
    println!("{:?}", &vector2.get(1));
    println!("{:?}", &vector2.get(10000)); //None

    let vector2 = vec!["abc".to_string(); 20];
    println!("{:?}", vector2);
}
