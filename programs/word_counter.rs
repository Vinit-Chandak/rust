use std::io;

fn count_words(input: &str) -> u32 {
    let mut count: u32 = 0;

    for ch in input.chars() {
        if ch == ' ' {
            count += 1;
        }
    }

    count + 1
}

fn main() {
    let mut input_string = String::new();

    println!("Enter the string(properly formatted): ");

    io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read a line.");
    
    let count = count_words(&input_string[..]);

    println!("The number of words in the given string is {}.", count);
}