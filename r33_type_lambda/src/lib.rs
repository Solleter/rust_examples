pub fn execute() {
    as_lambda();
    as_function();
}

fn as_lambda(){
    let x = 7;
    let print = || println!("{}", x);
    apply(print);
}

fn apply<F>(f: F) where F:Fn() {
    f();
}

fn as_function() {
    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}

fn call_me<F: Fn()>(f: F) {
    f()
}

fn function() {
    println!("I'm a function!");
}


