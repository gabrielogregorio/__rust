fn main() {
    println!("Hello, world!");

    let item: &u8 = lifetime(); // item is invalid
    let item2: u8 = lifetime2(); // item is valid
    let item3: &u8 = lifetime3(); // item is valid
}

fn lifetime() -> &u8 { //error, x not exists after run function
    let x: u8 = 2;
    &x
} // x is dropped


fn lifetime2() -> u8 { //not error, x are moved
    let x: u8 = 2;
    x
} // x are moved


fn lifetime3() -> &'static u8  { // not error
    let x: u8 = 2;
    &x
} // static not drop during execute programming... 