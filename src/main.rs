use hangman::Word;
use std::{io, collections::HashSet};

fn main() {



    let mut guess_list = HashSet::new();
    let mut thisone = Word::default();

    println!("{:?}", thisone);

    println!("WELCOME TO HANGMAN. GUESS THIS {} LETTER WORD. Type 1 to give up.:", thisone.length);

    // To do - process this so it looks like a nice string displayed to the user
    println!("{:?}", thisone.hidden);


    
    loop {
    let mut guess = String::new();

    match io::stdin().read_line(&mut guess){
        Ok(_) => {},
        Err(_) => println!("you guess is fucked"),
    };

    let guess = guess.trim();

    

    match guess {
        _ if guess.chars().any(|f| f=='1') => break,
        _ if guess.len()>1 => println!("One letter at a time, and no non-english chars pls pls"),
        _ if guess.chars().all(|c| !c.is_ascii_alphabetic()) => println!("No numbers"),
        _ => {

            let chars: Vec<char> = guess.chars().collect();
            let guess = chars[0].to_ascii_uppercase();
            if guess_list.contains(&guess) {
                println!("You already guessed the letter {guess}");
            } else {
                guess_list.insert(guess); // add the guess to the hash set
                Word::check_guess(&mut thisone, guess); // check if it's in the word
            }



        },

    

    }

    println!("{:?}", thisone.hidden);

    println!("{:?}", guess_list);
    }





}
