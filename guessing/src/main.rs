//Guessing game program : Rust Book Chapter 2
//for reading and writing from std io
use std::io::{self,Write};
//for random number generator
use rand::Rng;
//for getting the < > == enum for comparing guess and secret number
use std::cmp::Ordering;

fn main() {
    // user guess
    //
    //init the random number generator
    let mut rng = rand::thread_rng();
    // assign default value to the secret number
    let mut _guess:i32 = 0;
    // generate random number between 1 and 100
    let secret_no:i32 = rng.gen_range(0,100);
    // prepare a new string to get data from stdio
    let mut guess=String::new();
    //println!("secret number is {}",secret_no);
    
    println!("Guess a number between 1 and 100 :999 to quit");

    // do the below steps forever or come out if it executes a break
    loop
    {
        //do not use without clearling the variable
        guess.clear();
        println!("Enter value:");
        //flush the stdout to avoid reading old data
        io::stdout().flush().expect("Issue in flushing");
        //read the user input 
        io::stdin().read_line(&mut guess).expect("error found");
        // use the same variable name and create a new variable of integer type. 
        // use the same varaible in a match(case) to find if the user input is valid
        let guess:i32  = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed :{guess}");
        // once we have a valid number use the same in a match to find
        // if the number is low/high/eq to sectet number
        match guess.cmp(&secret_no)
        {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => { println!("You Win !!");
                break;
            }
        }
        
    }
}
