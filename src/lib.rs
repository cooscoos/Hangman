use std::fmt::Display;
use std::io;

mod player;
use player::Player;
mod words;
use words::Word;

pub fn new_game() -> (Word, Player) {
    // Initalise new player and a new word to guess
    let player1 = Player::default();
    let the_word = Word::default();

    println!(
        "\n\n ~~=== WELCOME TO HANGMAN ===~~ \n\n  Guess this {} letter word. Type 1 to give up.",
        the_word.length
    );

    (the_word, player1)
}

pub fn game_loop(the_word: &mut Word, player1: &mut Player) -> bool {
    // To do, organise this into something nicer, e.g. https://docs.rs/cli-grid/0.1.2/cli_grid/
    println!(
        "\n Mystery word: \n {:?} \n\n Guess a letter (type 1 to give up):",
        the_word.display_hidden()
    );

    let new_guess = get_player_input();

    match new_guess {
        _ if new_guess.to_lowercase() == "gfy" => {
            println!("\x1b[91m\n\n NO YOU GFY. \n\n PEW PEW!\x1b[0m");
            player1.kill();
        } // you could match several expletives with a list
        _ if new_guess.chars().any(|f| f == '1') => player1.kill(),
        _ if new_guess.len() > 1 => {
            println!("\x1b[95mOnly input one letter at a time, don't use non-english or special characters.\x1b[0m")
        }
        _ if new_guess.chars().all(|c| !c.is_ascii_alphabetic()) => {
            println!("\x1b[95mNo numbers (except 1 to give up). No symbols or tomfoolery please.\x1b[0m")
        }
        _ => {
            let chars: Vec<char> = new_guess.chars().collect();
            let guess = chars[0].to_ascii_uppercase();
            if player1.guessed_already(guess) {
                println!("\x1b[95mYou already guessed the letter {guess}!\x1b[0m");
            } else {
                player1.add_guess(guess); // add the guess to the hash set
                Word::check_guess(the_word, player1, guess); // check if it's in the word
            }
        }
    }

    player1.display_man(); // display the hangman

    // Display guessed words
    println!("Guessed letters: {:?}", player1.display_guesses());

    // Check if player dead
    if player1.is_dead() {
        println!("\x1b[91mYOU HANGED. GAME OVER. \x1b[0m");
        println!(
            "The word you failed to guess was: \x1b[96m{} \x1b[0m",
            the_word.show_answer()
        );
        return true;
    }

    // If there are no more underscores left in the hidden word, the player has won
    if !the_word.hidden.iter().any(|c| *c == '_') {
        println!("\x1b[93mYOU WIN!!!\x1b[0m");
        println!(
            "The word you guesssed correctly was: \x1b[96m{} \x1b[0m",
            the_word.show_answer()
        );
        return true;
    }
    false
}

pub fn get_player_input() -> String {
    let mut guess = String::new();
    match io::stdin().read_line(&mut guess) {
        Ok(_) => {}
        Err(_) => println!("ERROR: Your guess has somehow broken hangman."),
    };

    let guess = guess.trim();

    guess.to_string()
}

pub fn run_game() {
    let (mut the_word, mut player1) = new_game();

    loop {
        match game_loop(&mut the_word, &mut player1) {
            true => break,
            false => (),
        };
    }

    println!("Type y to play again, anything else to quit.");
    let player_input = get_player_input();
    match player_input {
        _ if player_input.to_lowercase() == "y" => run_game(),
        _ => (),
    }
}

pub fn list_with_spaces<T: Display>(input_letters: &[T]) -> String {
    // put some comma spaces between every guessed letter to make it more readable
    // create the spaces then zip them with the hidden underscores
    let spaces = vec![' '; input_letters.len()];

    let n = input_letters
        .iter()
        .zip(spaces)
        .map(|(c, space)| format!("{}{}", c, space))
        .collect::<String>();

    n
}
