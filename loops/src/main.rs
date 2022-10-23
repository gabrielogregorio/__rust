fn main() {
    // qtd
    for x in 0..10 {
        println!("Hello {}", x);
    }

    // while...
    let mut count = 0;
    while count < 10 {
        count = count + 1;
        println!("whiling, count = {}", count);
    }

    // looping...
    let mut count2 = 0;
    loop {
        count2 = count2 + 1;
        println!("start block = {}", count2);

        if count2 == 998 {
            continue;
        }

        println!("end block");

        if count2 == 999 {
            println!("stop loop");
            break;
        }
    }
}
