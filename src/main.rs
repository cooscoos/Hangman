use hangman::{Player, Word};
use std::{collections::HashSet, io};

fn main() {
    let mut player1 = Player::default();

    let mut guess_list = HashSet::new();
    let mut the_word = Word::default();

    println!(
        "\n\n ~~=== WELCOME TO HANGMAN ===~~ \n\n  Guess this {} letter word. Type 1 to give up.",
        the_word.length
    );

    loop {
        // To do, organise this into something nicer, e.g. https://docs.rs/cli-grid/0.1.2/cli_grid/

        println!("\n Word: ");
        println!("{:?}", the_word.display_hidden());
        println!("\n Guess a letter: ");
        let mut guess = String::new();

        match io::stdin().read_line(&mut guess) {
            Ok(_) => {}
            Err(_) => println!("you guess is fucked"),
        };

        let guess = guess.trim();

        match guess {
            _ if guess.to_lowercase() == "gfy" => {println!("YOU GFY"); player1.kill();}, // you could match several expletives with a list
            _ if guess.chars().any(|f| f == '1') => break,
            _ if guess.len() > 1 => {
                println!("One letter at a time, don't use non-english or special characters.")
            }
            _ if guess.chars().all(|c| !c.is_ascii_alphabetic()) => println!("No numbers, symbols or tomfoolery please."),
            _ => {
                let chars: Vec<char> = guess.chars().collect();
                let guess = chars[0].to_ascii_uppercase();
                if guess_list.contains(&guess) {
                    println!("You already guessed the letter {guess}!");
                } else {
                    guess_list.insert(guess); // add the guess to the hash set
                    Word::check_guess(&mut the_word, &mut player1, guess); // check if it's in the word
                }
            }
        }

        player1.display_man(); // display the hangman

        // check if player dead
        if player1.is_dead() {
            println!("GAME OVER");
            println!("The word you failed to guess was: {}", the_word.show_answer());
            break; // is this the proper way to quit from main?
        }

        // If there are no more underscores left in the hidden word, the player has won
        if !the_word.hidden.iter().any(|c| *c == '_') {
            println!("YOU WIN!!!");
            break; // is this the proper way to quit from main?
        }
    } // end of game loop
}
