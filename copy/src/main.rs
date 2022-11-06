fn main() {
    // Strings are in the Heap, it is more expensive to copy, so Rust does not do the deep copy, it just invalidates the pointer of the original variable and the new variable will receive the pointer of the old one, which was invalidated
    // // can use clone for deep copy
    let mut v1: String = String::from("ABC");
    println!("reference v1 => {} => {:p}", &v1, &v1); // v1 is here 0x7ffe33df7020

    let v2: String = v1; // v1 has invalidate, but your position is preserved... But not accessible
    println!("reference v2 => {} => {:p}", &v2, &v2); // v2 is here 0x7ffe33df70a0, not is some pointer from 1.

    v1 = String::from("ABC");
    println!("reference v1 => {} => {:p}", &v1, &v1); // v1 returns 0x7ffe33df7020, pointer exists

    // for simple value, difference betheween copy and deepcopy is desprezible... this is copia raza
    // atributions are copy
    // in compilations, size are conhecidos
    let v3: i8 = 123;
    let v4: i8 = v3;
    println!("reference v3 => {:p} => v4 => {:p}", &v3, &v4); // 0x7ffea1e5aa2e => v4 => 0x7ffea1e5aa2f

    // rust has special type,  trait Copy...
    // all integers(u32, example).
    // all boolean.
    // all char.
    // all floats.
    // all tuples with all types copy

    // by default, trait copy make copy on reatribute and not move variable in reatribuition

    // FUNCTIONS

    let s  = String::from("abc");
    let x = 1;

    takes_possession(s); // rust running drop métod for invalid access (denied only?)
    take_copy(x);

    // s MOVED
    println!("{}",  x)
}


fn takes_possession(any: String) {
    println!("{}", any);
}

fn take_copy(any: i32) {
    println!("{}", any);    
}