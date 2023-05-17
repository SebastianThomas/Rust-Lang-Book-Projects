fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The new value of x is {x}");
    let y = x;
    // let z = &x;
    x = 7 / 3;
    
    println!("The newest value of x is {x}");
    println!("The newest value of y is {y}");
//    println!("The newest value of z is {z}");
}
