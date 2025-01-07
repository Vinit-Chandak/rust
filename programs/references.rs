fn main() {
    let s = String::from("hello");
    let slice = s[1..4];
    // let slice = &s[1..4];
    println!("{}", slice);
}
