fn main() {
    println!("Hello, world!");
    println!("First: {}", get_fib(1));
    println!("Second: {}", get_fib(2));
    println!("Third: {}", get_fib(3));
    println!("Fourth: {}", get_fib(4));
    println!("Fifth: {}", get_fib(5));
    println!("Tenth: {}", get_fib(10));
    println!("Fiftieth: {}", get_fib(30));
    
    for i in 30..50 {
        println!("{}: {} \t | {:#034b}", i, get_fib(i), get_fib(i));
    }
}

fn get_fib(mut n: u8) -> u32 {
    let mut first = 1;
    let mut second = 1;

    while n > 2 {
        let temp = first + second;
        first = second;
        second = temp;
        n -= 1;
    }

    second
}
