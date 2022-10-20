fn main() {
    let values = [ 23,34,65,87,98,103 ];
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
    for value in values {
    println!("{value} in c is {}",temp_f_to_c(value));
    }
}

fn test(a:u32,b:u32) -> u32
{
    a+b
}

fn temp_f_to_c(value:u32) -> f32
{
    (5.0/9.0)  *(value as f32 - 32.0)
}
