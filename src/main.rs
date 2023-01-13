// Password generator

use rand::distributions::{Alphanumeric, DistString};
use std::io;

fn main() {
    println!("Please enter a number for the lenght of your new password:");
    println!("[Must be a number, greater than 8 and below 255]");

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            input = input.trim_end().to_string();
            match input.parse::<u8>() {
                Ok(num) => match num {
                    9..=u8::MAX => {
                        println!(
                            "Your password with the lenght of {} is being generated...",
                            num
                        );
                        let pwd: String =
                            Alphanumeric.sample_string(&mut rand::thread_rng(), num.into());
                        println!("Your new password (Alphanumeric Distribution) is:");
                        println!("{}", pwd);
                    }
                    _ => {
                        println!("Please enter a number between 9 and 255.");
                    }
                },
                Err(_) => {
                    println!("Please enter a valid number.");
                }
            }
        }
        Err(_) => {
            println!("Please enter a valid number.");
        }
    }
}
