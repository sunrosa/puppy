use std::io::{BufRead, Write};

/// Options for [`choose_conditional`] that are displayed if [`show`](ConditionalOption::show) is enabled.
pub struct ConditionalOption {
    /// The text to display to the user if [`show`](ConditionalOption::show) is true.
    pub text: String,
    /// Display the [`text`](ConditionalOption::text) to the user as a choice.
    pub show: bool,
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
pub fn choose(options: Vec<&str>) -> usize {
    for (i, option) in options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    loop {
        // Prompt caret
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

/// Asks the user for their input through a prompt, returning their choice as the index of the vec of choices passed in. This function will repeat the prompt until the user provides a valid positive integer in range.
/// # Arguments
/// * `options` - The mandatory choices presented to the user.
/// * `conditional_options` - The options that will be presented to the user at the top of the options list if [`ConditionalOption::show`] is true.
/// # Design flaws
/// * [`choose_conditional`] outputs an ambiguous index, as it is difficult for the user of the function to determine which conditional options appear, and where.
pub fn choose_conditional(
    options: Vec<&str>,
    conditional_options: Vec<ConditionalOption>,
) -> usize {
    // Add enabled conditional options to beginning of displayed_options
    let mut displayed_options: Vec<String> = conditional_options
        .into_iter()
        .filter(|option| option.show)
        .map(|option| option.text)
        .collect();
    // Concatenate mandatory options to the end of displayed_options
    displayed_options.extend(
        options
            .into_iter()
            .map(|option| String::from(option))
            .collect::<Vec<String>>(),
    );

    for (i, option) in displayed_options.iter().enumerate() {
        println!("{}. {}", i + 1, option);
    }
    loop {
        // Prompt caret
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
                if selection <= displayed_options.len() {
                    break selection - 1;
                } else {
                    println!(
                        "Please enter a valid number between 1 and {}.",
                        displayed_options.len()
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
