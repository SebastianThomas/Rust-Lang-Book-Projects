fn main() {
    let s1 = String::from("Hi my string");
    let s2 = "Hi";
    let s3 = s1.clone();

    println!("{} hi", &s1);
    println!("{}", s2);
    println!("{}", s3);

    let mut str = get_str();
    println!("{}", str);
    str = get_str();
    println!("{}", str);

    println!("{}", get_str_slice(&String::from("hi"), "you".to_string()));
}

fn get_str() -> String {
    String::from("hi")
}

fn get_str_slice(s: &str, s2: String) -> String {
    s2 + s
}

