use std::io;

fn main() {
    println!("How many points is your team favored by?");
    
    let mut user_input = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    // println!("You Entered: {}", user_input.trim());
    
    let favored_points: i32 = user_input
        .trim()
        .parse()
        .expect("Failed to convert to i32");

    println!("Parsed number: {}", favored_points);
}
