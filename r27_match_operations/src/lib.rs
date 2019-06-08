#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}


pub fn execute() {
    match_pair();
    match_enum();
    match_ref();
    match_struct();
    match_guard();
    match_binding();
    match_option();
    match_if_let_enum();
    match_loop_let();
    match_while_let();
}

fn match_pair() {
    let pair = (0, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (0, y) => println!("First is '0' and 'y' is {:?}", y),
        (x, 0) => println!("'x' is '{:?}' and last is '0'", x),
        _ => println!("It doesn't matter what they are"),
    }
}

fn match_enum() {
    let color = Color::RGB(122, 17, 40);
    println!("What color it is?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue"),
        Color::Green => println!("The color is Green"),
        Color::RGB(r,g, b) => println!("Red: {}, Green:{}, Blue: {}", r,g,b),
        Color::HSV(r,g, b) => println!("R:{}, G:{}, B:{}", r,g,b),
        Color::HSL(r,g,b) => println!("R:{}, G:{}, B:{}", r,g,b),
        Color::CMY(r,g,b) => println!("R:{}, G:{}, B:{}", r, g, b),
        Color::CMYK(r, g, b, a) => println!("R:{}, G:{}, B:{}, A:{}", r,g,b,a),
    }
}

fn match_ref() {
    let reference = &4;
    match reference {
        &val => println!("Got a value via destructing: {:?}", val),
    }

    println!("reference: {:?}, *reference: {:?}", reference, *reference);

    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;
    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a refrence to a value: {:?}", r),
    }

    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10.'mut_value': {:?}", m);
        },
    }

    println!("mut_value: {}", mut_value);
}

fn match_struct() {
    struct Foo { x: (u32, u32), y: u32 }

    let foo = Foo{ x: (1, 2,), y: 3 };
    let Foo {x:(a, b), y} = foo;

    println!("a = {}, b = {}, y = {}", a, b, y);

    let Foo { y: i, x: j} = foo;
    println!("i = {:?}, j = {:?}", i, j);

    let Foo { y, .. } = foo;
    println!("y = {}", y);
}

fn match_guard() {
    let pair = (2, -2);
    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

fn age() -> u32 {
    90
}

fn match_binding() {
    println!("Tell me type of person you are");
    match age() {
        0 => println!("I'm not born yet I guess"),
        n @ 1 ... 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ... 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
}

fn match_option() {
    let number = Some(7);
    let letter: Option<i32> = None;
    let emotion: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        println!("Didn't matched a number. Let's go with a letter!");
    }

    let i_like_letters = false;
    if let Some(i) = emotion {
        println!("Matched {:?}!", i);
    } else if i_like_letters {
        println!("Didn't matched a number, Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with a emothion: :)");
    }
}

fn match_if_let_enum() {
    enum Foo {
        Bar,
        Baz,
        Qux(u32),
    }

    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a {
        println!("a is a foobar");
    }

    if let Foo::Bar = b {
        println!("b is a foobar");
    }

    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }
}

fn match_loop_let() {
    let mut optional = Some(0);
    loop {
        match optional {
            Some(i) => {

                    if i > 9 {
                        println!("Greater than 9, quit!");
                        optional = None;
                    }else {
                        println!("'i' is '{:?}'. Try again.", i);
                        optional = Some(i + 1);
                    }

            },
            _ => {
                println!("End");
                break;
            }
        }
    }
}

fn match_while_let() {
    println!("match while let ========");
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("i is {:?}", i);
            optional = Some(i + 1);
        }
    }
}