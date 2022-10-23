fn main() {
    let mut name: &str = "string with size len";
    let mut body: String = String::from("string with infinit? len");

    println!("name has capacity {}", name.len());
    println!("body has capacity {}", body.len());

    // name.push_str("concatenate String"); // error, methods not found n &str
    body.push_str("concatenate String");
    println!("body has capacity {}", body.len());
}
