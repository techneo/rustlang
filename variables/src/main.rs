fn main() {
    const PI:f64 = 3.14;
    let mut area = 0.0;
    let x:i32 =  20;
    let mut y:i32=77;
    //x = 10;
    println!("old val of {y}");
    y = 11;
    println!("Values are {x} and {y}");
    area = PI * (x ^ 2) as f64 ;
    println!("Area = {area}");
    let y:u32 = {
        let c = 22;
        c + 2
    };
    println!("value of {}",y);
    println!("test return {}",test(4,5));
}

fn test(a:u32,b:u32) -> u32
{
    a+b
}
