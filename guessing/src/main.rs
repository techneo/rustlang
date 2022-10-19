use std::io::{self,Write};
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    // user guess
    //
    let mut rng = rand::thread_rng();
    let mut _guess:i32 = 0;
    // random number
    let secret_no:i32 = rng.gen_range(0,100);
    let mut guess=String::new();
    println!("secret number is {}",secret_no);
    
    println!("Guess a number between 1 and 100 :999 to quit");


    loop
    {
        //do not use without clearling the variable
        guess.clear();
        println!("Enter value:");
        io::stdout().flush().expect("Issue in flushing");
        io::stdin().read_line(&mut guess).expect("error found");
        let guess:i32  = guess.trim().parse().expect("Incorrect value entered by user");
        match guess.cmp(&secret_no)
        {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("To Big!"),
            Ordering::Equal => { println!("You Win !!");
                break;
            }
        }
        
    }
    // need a way to get a random number between 1 and 100
    //
    
}
