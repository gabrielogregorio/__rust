struct Block {
    width: u8,
    height: u8,
}

impl Block {
    fn area(&self) -> u8 {
        return self.width * self.height;
    }
}

fn main() {
    let my_block = Block {
        width: 4,
        height: 6,
    };

    println!("here {}m²", my_block.area());
}
