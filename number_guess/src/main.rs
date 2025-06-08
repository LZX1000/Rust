use std::io::Write;
use std::str::FromStr;

use rand::Rng;

macro_rules! clear_screen {
    ($soft:expr) => {
        clear_screen($soft);
    };
    () => {
        clear_screen(false);
    };
}

fn clear_screen(soft: bool) {
    if soft {
        print!("\x1B[2J\x1B[1;1H");
    } else {
        print!("{esc}c", esc = 27 as char);
    }
    std::io::stdout().flush().unwrap();
}


macro_rules! input {
    ($prompt:expr, $type:ty, $default:expr) => {
        input::<$type>($prompt, Some($default))
    };
    ($prompt:expr, $type:ty) => {
        input::<$type>($prompt, None)
    };
    ($prompt:expr) => {
        input::<String>($prompt, None)
    };
    ($type:ty) => {
        input::<$type>("", None)
    };
    ($default:expr) => {
        input::<String>("", Some($default))
    };
    () => {
        input::<String>("", None)
    };
}

fn input<T: FromStr>(prompt: &str, default: Option<T>) -> T
where
    T::Err: std::fmt::Debug
{
    
    if !prompt.is_empty() {
        print!("{}", prompt);
        std::io::stdout().flush().unwrap();
    }
    let mut buffer: String = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed to read line");

    buffer = buffer.trim().to_string();
    if buffer.is_empty() {
        if let Some(default_value) = default {
            return default_value;
        } else {
            panic!("No input provided and no default value specified.");
        }
    }
    
    buffer.parse::<T>().expect("Failed to parse input")
}


fn main() {
    let mut rng = rand::thread_rng();

    let min: u16 = 1;
    let max: u16 = 100;

    let mut highscore: Option<u16> = None;

    clear_screen!();

    let mut playing: bool = true;
    while playing {
        let heading: String = format!(
            "Best: {}",
            match highscore {
                Some(score) => score.to_string(),
                None => "None".to_string(),
            }
        );
        let secret_num: u16 = rng.gen_range(min..=max);

        let mut message: String = String::new();

        let mut min_found: u16 = min;
        let mut max_found: u16 = max;
        let mut guesses: u16 = 0;

        loop {
            let bounds: String = format!("{}: ({}, {})", guesses, min_found, max_found);

            guesses += 1;

            println!("{}\n{}\n{}", heading, bounds, message);
            let guess: String = input!("\nGuess a number: ");

            if guess.is_empty() || guess.eq_ignore_ascii_case("exit") {
                clear_screen!();
                println!("Goodbye.");
                playing = false;
                break
            }

            message.clear();
            match guess.parse::<u16>() {
                Ok(n) => {
                    clear_screen!(true);
                    if n < secret_num {
                        message = "Too low!".to_string();
                        if n > min_found {
                            min_found = n;
                        }
                    } else if n > secret_num {
                        message = "Too high!".to_string();
                        if n < max_found {
                            max_found = n;
                        }
                    } else {
                        // message = "Correct!".to_string(); //Never read
                        break;
                    }
                },
                Err(_) => {
                    clear_screen!(true);
                    message = "Please enter a valid number.".to_string();
                    continue;
                }
            }
        }

        clear_screen!();
        println!("You guessed {} in {} guesses!\n", secret_num, guesses);

        match highscore {
            Some(score) if guesses < score => {
                highscore = Some(guesses);
            }
            None => {
                highscore = Some(guesses);
            }
            _ => {}
        }

    }
}