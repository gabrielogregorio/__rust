use std::mem;

const MY_CONST_ABC: &str = "santos";
const NUMBER_2: u8 = 6;

fn main() {
    let any_variable = "abc, faz inferenia";
    let any_variable2: &str = "abc, faz inferenia2";
    let vectors: Vec<&str> = vec!["abc", "123"];
    let _not_unused_var_and_ignore: i8 = 1;

    println!("size {} is {}", any_variable, any_variable.len());

    println!(
        "{}, {} {} {:?}",
        any_variable, any_variable2, MY_CONST_ABC, vectors
    );
    println!("logic1 {}", !true);
    println!("logic2! {}", !true || false);
    println!("logic3! {}", !true && false || true);

    println!("Integer positive only {}", 450u32 - 200);
    println!("integer real only {}", 450i32 - 200);
    println!("integer real only {}", 10i32 - 200);

    println!("numbers {}", 10f64 + 200.0 - 2.0 / 199.0);
    println!("millions {}", 10_000_000_000u64);

    if any_variable == "abc" || (NUMBER_2 * NUMBER_2) == 36 {
        println!("Here");
    }

    if !(1 == 2) {
        println!("Here 2");
    }

    if 10 / 2 == 5 {
        println!("Here 3");
    }

    let message = if 1f32 > 0.1f32 {
        "this not is"
    } else {
        "this is"
    };
    println!("{}", message);

    let a: i8 = 1;
    let b: u8 = 1;
    let c: i32 = 1;
    let d: char = 'a';
    let e: &str = "a";
    let f: String = String::from("a");
    let g: isize = 123; // size of reference local memory
    let h: bool = true;

    println!("a = '{}', i8     - size {} bytes", a, mem::size_of_val(&a));
    println!("b = '{}', u8     - size {} bytes", b, mem::size_of_val(&b));
    println!("c = '{}', i32    - size {} bytes", c, mem::size_of_val(&c));
    println!("d = '{}', char   - size {} bytes", d, mem::size_of_val(&d));
    println!("e = '{}', &str   - size {} bytes", e, mem::size_of_val(&e));
    println!("f = '{}', String - size {} bytes", f, mem::size_of_val(&f));
    println!("g = '{}', String - size {} bytes", g, mem::size_of_val(&g));
    println!("h = '{}', String - size {} bytes", h, mem::size_of_val(&h));

    let squared = i32::pow(3i32, 3);
    println!("{}", squared);

    let squared_int = f32::powi(3f32, 3);
    println!("{}", squared_int);

    let squared_float = f32::powf(3f32, 3.5);
    println!("{}", squared_float);

    let mut test: String = String::from("texto");
    test.push_str("string");
    let immutable = &test;
    println!("{} {}", immutable, test);
    // test.push_str("ss"); // ERROR immutable borrow later used here
    println!("{}", immutable);

    return;
}
