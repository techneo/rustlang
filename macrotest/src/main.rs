/*
Another use of !
In Rust, the ! symbol is used to indicate 
that a function has the "never" return type. 
This means that the function is guaranteed to 
always panic, loop indefinitely, or exit the process. 
Functions with this return type are called "diverging functions".
 */

//  a macro that takes an argument and doubles it 
macro_rules! grow {
    ($val:expr) => {
        $val*2
    }
}

// a macro that ruturns 0 
macro_rules! someval {
    () => {
        0
    }
}

macro_rules! twoargs {
    ($t1:expr,$t2:expr) => {
        println!("Val1 {} val2 {}",$t1,$t2);
    }
}


fn main() {
    println!("Hello, world! {} {} ",grow!(5),someval!());
    twoargs!(4,5);
}
