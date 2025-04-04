fn main() {

    let hero_name = "Van Damme".to_string();        // String
    println!("The hero name is: {:#?}", hero_name);

    let short_name = hero_name.replace("Van Damme", "JCVD ");   // String
    println!("The short name is: {:#?}", short_name);

    let position = String::from("Quarterback");     // String
    println!("The position is: {:#?}", position);

    let greetings = "Greetings from the team";   // String
    println!("The greetings is: {:#?}", greetings);

    let short_greetings = greetings.get(0..9).unwrap();  // &str
    println!("The short greetings is: {:#?}", short_greetings);

    let hello ="Hello";   // &str
    let hello_world ="Hello World".to_string();   // String
    let hello_ref = &hello_world;    // String
    

    println!("The hello is: {:#?}", hello);
    println!("The hello world is: {:#?}", hello_world);
    println!("The hello ref is: {:#?}", hello_ref);


}
