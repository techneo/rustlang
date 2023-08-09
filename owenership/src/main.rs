//new comment
//adding new comment
fn main() {
    println!("Hello, world!");
    {
        { // test_var not in scope
        let test_var = 20;   //test_var in scope
        println!("The value is live here {test_var}");
        } //test_var not in scope
        // try to access the variable here
        //println!("The value is live here {test_var}");

        {
        // The double colon :: operator allows us to namespace this particular 
        //from function under the String type rather than using some sort of name like string_from

        let mut s = String::from("hello");
        println!("Value of string is {s}");

        // not posible if s is not mutable
        s.push_str("world");
        println!("Value of string is {s}");
        } // Rust calls drop once the variable is out of scope.

        // will not work as the string has been deallocated from memory.
        // s.push_str("world");
        // println!("Value of string is {s}");
    }
}
