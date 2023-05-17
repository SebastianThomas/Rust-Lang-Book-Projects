fn main() {
    let x: &str = "ddff";
    println!("{}", first(longest(x, "str1")));
    println!("{}", first(x));
}

fn first<'a>(x: &'a str) -> &'a str {
    x
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

