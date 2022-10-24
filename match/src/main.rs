struct Color {
    r: u8,
    g: u8,
    b: u8,
}

fn main() {
    println!("Hello, world!");

    let algo: &str = "algo1";

    match algo {
        "algo" => println!("você disse algo"),
        "algo_outro" => {
            let q = "";
            println!("q {}", q);
        }
        // 10..=90 for example
        "2" | "3" | "4" => println!("você disse algo 2222"),
        "outro" => println!("você disse outro"),
        _ => println!("você disse ooops"),
    }

    let color: Color = Color { r: 2, g: 0, b: 0 };

    how_color_is(color);
}

fn how_color_is(c: Color) {
    match c {
        Color { r: 0, g: 0, b: any } => {
            println!("herreee is blue {}", any);
        }
        Color { r: any, g: 0, b: 0 } => {
            println!("herreee is red {}", any);
        }
        Color { r: 0, g: any, b: 0 } => {
            println!("herreee is green {}", any);
        }
        _ => {
            println!("is other");
        }
    }
}
