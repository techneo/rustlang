struct Greetings {
    name: String,
}



//this is a constructor
impl Greetings {
    // &str is a string slice
    fn new(name: &str) -> Self {
        Greetings {
            name : name.to_string(),
        }
    }
}



fn main() {

    // this is ugly
    // we can use constructors to make it simple
    //let greet = Greetings {
    //    name : "linz".to_string(),
    //};

    let greet1 = Greetings::new("Suren");

    //println!("Hello {}",greet.name);
    println!("Hello {}",greet1.name);
}
