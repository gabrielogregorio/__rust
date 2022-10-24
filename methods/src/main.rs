enum TypeQuad {
    Quad = 1,
    Rect = 2,
}

#[derive(Debug)]
enum MagicValue {
    Number(u8),
    Str(String),
    Struct { x: u8, y: u8 },
}

type CopyTypeQuad = TypeQuad;

struct Block {
    width: u8,
    height: u8,
    type_item: TypeQuad,
}

impl Block {
    fn area(&self) -> u8 {
        return self.width * self.height + self.type_item as u8;
    }
}

enum Option<T> {
    Some(T),
    None,
}

fn main() {
    let my_block = Block {
        width: 4,
        height: 6,
        type_item: CopyTypeQuad::Quad,
    };

    println!("here {}m²", my_block.area());

    let magic_item = MagicValue::Number(2);
    let magic_item_2 = MagicValue::Str(String::from(""));
    let magic_item_3 = MagicValue::Struct { x: 1, y: 2 };

    println!("{:?}, {:?}, {:?}", magic_item, magic_item_2, magic_item_3);

    let any = Option::Some("aaaa");
    let any_2 = Option::Some(false);
    let any_3 = Option::Some(333333333);
    let any_4 = Option::Some(1);
}
