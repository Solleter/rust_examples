
pub fn public_function(){
    println!("Called rary's public function");
}

fn private_function() {
    println!("Called rary's private_function ()");
}

pub fn indirect_access() {
    println!("called rary's indirect_access");
    private_function();
}