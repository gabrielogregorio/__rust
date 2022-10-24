fn main() {
    // qtd
    for x in 0..10 {
        println!("Hello {}", x);
    }

    let games = vec!["gta3", "gta sa", "gta V", "horizon zero down"];

    for game in games {
        println!("game is {}", game);
    }

    let array: [&str; 2] = ["abc", "def"];

    for array_item in array.iter() {
        println!("game with iter is {}", array_item);
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

        if count2 == 14 {
            continue;
        }

        println!("end block");

        if count2 == 15 {
            println!("stop loop");
            break;
        }
    }

    // loop and modify
    let mut items_to_sum: Vec<i32> = vec![10, 20, 30];
    for item in &mut items_to_sum {
        // get object by reference and modify
        *item *= 10;
        println!("New value is {}", item);
    }
    println!("new vector {:?}", items_to_sum);
}
