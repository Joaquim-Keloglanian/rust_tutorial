fn loops() {
    // basic_loop();
    // labeled_loops();
    // while_loop();
    // collections_loop();
    // for_loop();
    range_loop();
}

fn labeled_loops() {
    let mut count: i32 = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn basic_loop() {
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn while_loop() {
    let mut number: i32 = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn collections_loop() {
    let a: [u32; 5] = [10, 20, 30, 40, 50];
    let mut index: usize = 0;

    while index < 5 {
        println!("The value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    let array: [u32; 4] = [1, 2, 3, 4];
    let mut index: usize = 0;

    for element in array {
        println!("The value is: {element}");
    }
}

fn range_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
