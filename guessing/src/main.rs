use std::io::{self,Write};
use rand::Rng;

fn main() {
    // user guess
    //
    let mut rng = rand::thread_rng();
    let mut _guess:i32 = 0;
    // random number
    let _secret_no:i32 = rng.gen_range(0,100);
    let mut guess=String::new();
    println!("secret number is {}",_secret_no);
    
    println!("Guess a number between 1 and 100 :999 to quit");


    loop
    {
        //do not use without clearling the variable
        guess.clear();
        println!("Enter value:");
        io::stdout().flush();
        io::stdin().read_line(&mut guess).expect("error found");
        let mut guess:i32  = guess.trim().parse().expect("Incorrect value entered by user");
        if guess == 999 || guess == _secret_no
        {
            break;
        }
        if guess == _secret_no
        {
            println!("You got it !!");
        }
        
    }
    // need a way to get a random number between 1 and 100
    //
    
}
