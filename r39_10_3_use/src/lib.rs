use deeply::nested::function as other_function;

fn function() {
    println!("Called function()");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("Called deeply nested function");
        }
    }
}

pub fn execute() {
    other_function();

    println!("Entering block");

    {
        use deeply::nested::function;
        function();
        println!("Leaveing block");
    }

    function();
}
