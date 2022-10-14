use std::io;
fn main() {
    // user guess
    //
    let mut input=String::new();
    let mut guess:i32 = 0;
    // random number
    let secret_no:i32 = 0;
    let mut choice=String::new();

    println!("Do you want to play:");
    io::stdin().read_line(&mut choice).expect("error found");
//    if (choice != 'y')

    println!("Enter value:");
    io::stdin().read(&mut input).expect("error found");

    println!("{}",guess);
    // need a way to get a random number between 1 and 100
    //
    
}
