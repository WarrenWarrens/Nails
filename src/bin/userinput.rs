use std::io;

fn main() {
    loop {
        println!("Please enter a number:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Match the Result of .parse()
        let number: i32 = match input.trim().parse() {
            Ok(num) => num, // If successful, bind the value to 'number'
            Err(_) => {     // If it fails (e.g., they typed a letter), catch the error
                println!("That's not a valid number! Please try again.\n");
                continue;   // Skips the rest of the loop and starts over from the top
            }
        };

        // If we get here, we guarantee 'number' is a valid i32
        if number % 2 == 0 {
            println!("{} is even.", number);
        } else {
            println!("{} is odd.", number);
        }

        break; // Exit the loop now that we have successfully processed the number
    }
}