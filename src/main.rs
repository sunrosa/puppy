use std::io::{BufRead, Write};

fn main() {
    match choose(vec!["Yes", "No", "Maybe"]) {
        0 => println!("You chose yes."),
        1 => println!("You chose no."),
        2 => println!("You chose maybe."),
        _ => panic!(),
    }
}

/// Asks the user for their input through a prompt, returning their choice as the index of the vec of choices passed in. This function will repeat the prompt until the user provides a valid positive integer in range.
/// # Arguments
/// `options` - The choices you present to the user.
/// # Examples
/// ```no_run
/// fn main() {
///    match choose(vec!["Yes", "No", "Maybe"]) {
///        0 => println!("You chose yes."),
///        1 => println!("You chose no."),
///        2 => println!("You chose maybe."),
///        _ => panic!(),
///    }
///}
/// ```
fn choose(options: Vec<&str>) -> usize {
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();
        match std::io::stdin()
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .parse::<usize>()
        {
            Ok(selection) => {
                if selection <= options.len() {
                    break selection - 1;
                } else {
                    println!(
                        "Please enter a valid number between 1 and {}.",
                        options.len()
                    );
                }
            }
            Err(_) => {
                println!("Please enter a valid positive integer.");
                continue;
            }
        }
    }
}
