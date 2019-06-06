#![allow(unreachable_code)]
pub fn execute() {
    loop_function();
    nested_loop();
    loop_return_value();
    while_loop();
    for_iter();
    for_info_iter();
    for_iter_mut();
}

fn loop_function() {
    let mut count = 0u32;
    println!("Let's count untile infinity!");
    loop{
        count += 1;
        if count == 3 {
            println!("Three");
            continue;
        }

        println!("{}", count);
        if count == 5 {
            println!("OK, that's enough");
            break;
        }
    }
}

fn nested_loop() {
    'outer: loop {
        println!("Entered the out loop");

        'inner: loop {
            println!("ENtered the inner loop");
            break 'outer
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

fn loop_return_value() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
}

fn while_loop() {
    let mut n = 1;
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
          println!("{}", n);
        }

        n += 1;
    }
}

fn for_iter () {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("THere is a restacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn for_info_iter() {
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferries" => println!("THere is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}

fn for_iter_mut() {
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        match name {
            &mut "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
}