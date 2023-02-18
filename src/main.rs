use std::io::{BufRead, Write};

struct GameData {
    has_sledgehammer: bool,
}

fn main() {
    let data = GameData {
        has_sledgehammer: false,
    };

    println!("You find yourself in a dark void. The air is cool. You pick up a faint dripping sound in the distance.");
    starting_room(data);
}

fn starting_room(game_data: GameData) {
    match choose(vec!["Feel ground", "Explore area"]) {
        0 => {
            // Feel ground
            println!(
                "The ground is damp. Crumbling stone bricks line the floor, sprinkled with moss."
            );
            starting_room(game_data);
        }
        1 => {
            // Explore area
            println!("You have a walk around. It's too dark to see anything. Your outstretched hand collides with something. It's a wall.");
            match choose(vec!["Go left", "Go right"]) {
                0 => {
                    // Go left
                    println!("You turn left, running your right hand against the wall to keep a bearing, before stumbling upon a chest.");
                    starting_room_chest(game_data);
                }
                1 => {
                    // Go right
                    println!("You turn right, running your left hand against the wall to keep a bearing. You take ten paces, nearly taking an eleventh into a gaping crack in the floor before you, catching yourself at the last moment, throwing your weight to the right, and falling over. At least you didn't fall into the hole.");
                    starting_room_hole(game_data);
                }
                _ => panic!(),
            }
        }
        _ => panic!(),
    }
}

fn starting_room_chest(game_data: GameData) {
    match choose(vec!["Examine chest", "Open chest", "Leave chest"]) {
        0 => {
            // Examine chest
            println!("The chest is dusty. Two of the metal bands have come loose from the lid and bend in awkward directions.")
        }
        1 => {
            // Open chest
            println!("You open the chest and find an old sledgehammer. With your strength, you're barely able to swing it. You decide to bring it with you regardless.");
        }
        2 => {
            // Leave chest
            todo!();
        }
        _ => panic!(),
    }
}

fn starting_room_hole(game_data: GameData) {
    match choose(vec![
        "Continue around the hole",
        "Jump into the hole anyway",
        "Examine hole",
    ]) {
        0 => {
            // Continue around the hole
            todo!();
        }
        1 => {
            // Jump into the hole anyway
            println!("You steel yourself, and leap into the hole, falling for what feels like eternity. You hear your boots clap and echo against the stone floor as you're reuinited with the ground.");
            println!("Is that... light? You see a faint orange glow maybe fifty meters away.");
            todo!();
        }
        2 => {
            // Examine hole
            println!("The hole has a cracked and jagged edge. Some of the bricks have fallen through, some others are split and cracked halfway through. It appears something heavy slammed into this floor here.");
            starting_room_hole(game_data);
        }
        _ => panic!(),
    }
}

/// Options for [`choose_conditional`] that are displayed if [`show`](ConditionalOption::show) is enabled.
struct ConditionalOption {
    /// The text to display to the user if [`show`](ConditionalOption::show) is true.
    text: String,
    /// Display the [`text`](ConditionalOption::text) to the user as a choice.
    show: bool,
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
fn choose_conditional(options: Vec<&str>, conditional_options: Vec<ConditionalOption>) -> usize {
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
