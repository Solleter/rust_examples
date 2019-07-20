fn function() {
    println!("called function()");
}

mod cool {
    pub fn function() {
        println!("called cool::function()");
    }
}

mod my {
    fn function() {
        println!("called my::function()");
    }

    mod cool {
        pub fn function() {
            println!("caled my::cool::function()");
        }
    }

    pub fn indirect_call() {
        println!("called my::indriect_call()");
        self::function();
        function();

        self::cool::function();
        super::function();

        {
            use cool::function as root_function;
            root_function();
        }
    }
}

pub fn execute() {
    my::indirect_call();
}