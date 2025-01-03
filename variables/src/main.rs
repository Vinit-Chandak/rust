fn main() {
    const Y: i32 = 60 * 3;
    let mut x: u64 = 3;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);

    let y = 32;
    println!("The value of y is: {y}");
    let mut y = 23;
    println!("The value of y is: {y}");
    y = 22;
    println!("The value of y is: {y}");

    let mut trun = 5 / 3;
    println!("trun: {trun}");
    trun = 5 / 2;
    println!("trun: {trun}");
    trun = 5 / 6;
    println!("trun: {trun}");
    trun = 50000 / 50001;
    println!("trun: {trun}");
}
