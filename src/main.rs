use std::io::{BufRead, Write};

fn main() {
    choose(vec![
        "AAAA",
        "WAAAAAAAAAAA",
        "WAAAAAAAAAAHAHAHAHAHAAAAAAAAAAAA",
    ]);
}

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
                    break selection;
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
