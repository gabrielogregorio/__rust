use std::fmt::Debug;

#[derive(Debug)]

enum Option<T> {
    Some(T),
    None,
}

struct Magic<T> {
    value: T,
}

impl<T: Debug> Magic<T> {
    fn show_generics(&self) {
        println!("implementation generic {:?}", self.value)
    }

    fn show_other<U: Debug>(&self, extra: U) {
        println!("show extra {:?}", extra)
    }
}

fn any_item<T: std::fmt::Debug>(x: T) {
    println!("{:?}", x);
}

fn main() {
    let x: Option<String> = Option::Some("Oops".to_string());
    println!("> {:?}", x);

    let y: Magic<i32> = Magic { value: 280 };
    let z: Magic<String> = Magic {
        value: String::from("here"),
    };
    println!("> {:?} {:?}", y.value, z.value);

    any_item(25i8);
    any_item("Item");
    any_item('c');

    let generics_with_implementation: Magic<i8> = Magic { value: 2 };
    generics_with_implementation.show_generics();
    generics_with_implementation.show_other("my string".to_string());
}
