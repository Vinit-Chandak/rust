use std::io;

fn main() {
    //create a mutable string to store the input
    let mut input = String::new();

    println!("Enter n: ");

    // read an entire line from the input and append it to the string
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read a line."); // handle the error

    // trim the newline char and extra spaces
    // convert the input string to an integer
    let n: i32 = input.trim().parse().expect("Please enter a valid number.");

    let mut a = 1;
    let mut b = 1;
    let mut c = 0;
    
    if n == 1 {
        println!("1");
    }
    else if n == 2 {
        println!("1");
    }
    else {
        for _num in 2..n {
            c = a + b;
            a = b;
            b = c;
        }    
        println!("{c}");
    }
    
}