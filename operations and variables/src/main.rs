const MY_CONST_ABC: &str = "santos";

fn main() {
    let any_variable = "abc, faz inferenia";
    let any_variable2: &str = "abc, faz inferenia2";
    let vectors: Vec<&str> = vec!["abc", "123"];
    let _not_unused_var_and_ignore: i8 = 1;

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
}
