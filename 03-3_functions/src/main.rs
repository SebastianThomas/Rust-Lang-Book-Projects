fn main() {
    println!("Hello, world!");

    my_func();
    println!("{}", my_funcp(5));
//    my_funcp();
}

fn my_func() {
    println!("Coming from other function");
}

fn my_funcp(i: i32) -> i32 {
    println!("Coming from function with param {i}");
    2 * i
}
