use std::collections::HashMap;

fn call(number: &str) -> &str {
    "Calling OK"
}

fn hashmap_test() {
    let mut contacts = HashMap::new();

    contacts.insert("Daniel", "111111");
    contacts.insert("Ashley", "222222");
    contacts.insert("Katie", "333333");
    contacts.insert("Daniel", "111111");
    contacts.insert("Daniel", "111111");
    contacts.insert("Daniel", "111111");
    contacts.insert("Daniel", "111111");
    contacts.insert("Daniel", "111111");
    contacts.insert("Daniel", "111111");
    contacts.insert("Daniel", "111111");

   match contacts.get(&"Daniel") {
       Some(&number) => println!("Calling Daniel: {}", call(number)),
       _ => println!("Don't have Daniel's number."),
   }

    println!("============================");
    for(contact, &number) in contacts.iter() {
        println!("Calling {}: {}", contact, call(number));
    }


}

pub fn execute(){
    hashmap_test();
}