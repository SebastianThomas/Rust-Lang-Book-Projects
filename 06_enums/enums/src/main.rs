fn main() {
    println!("Hello, world!");
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
    let s: Option<String> = None;

    println!("{} {}", 
           some_number.unwrap(), 
           some_char.unwrap()
    );
    println!("{} {}",
           match absent_number {
               Some(v) => v,
               None => 0
           }, 
           s.unwrap_or_else(|| String::from("NONE")) 
    );
}

