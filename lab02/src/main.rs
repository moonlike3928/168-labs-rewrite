use std::io::{self, Write};

fn main() {
    let mut points = String::new();
    
    print!("How many points is your team favored by?: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut points)
        .expect("Failed to read line");
    
    let points = points.trim().parse::<f64>().expect("Couldn't parse points");

    print!("Is it a home game? (h/H): ");
    io::stdout().flush().unwrap();

    let mut home_input = String::new();
    io:: stdin()
        .read_line(&mut home_input)
        .expect("Failed to read line");
    
    let home_game = home_input.trim().eq_ignore_ascii_case("h");

    print!("Are you picking the Bengals? (y/Y): ");
    io::stdout().flush().unwrap();

    let mut bengals_input = String::new();
    io:: stdin()
        .read_line(&mut bengals_input)
        .expect("Failed to read line");
    
    let picked_bengals = bengals_input.trim().eq_ignore_ascii_case("y");
    if picked_bengals {
        println!("NO! NO! NO!");
    } else if points >= 7.5 {
        println!("SLAM IT!");
    } else if (points >= 3.0) && (home_game) {
        println!("Good bet");
    } else if points >= 3.0 {
        println!("Okay bet");
    } else if (points > 0.0) && (home_game) {
        println!("Highly risky");
    } else {
        println!("Don't bet!")
    }

}
