use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::list_with_spaces;
use crate::player::Player;

#[derive(Debug)]
pub struct Word {
    revealed: String,
    pub hidden: Vec<char>,
    pub length: usize,
}

impl Word {
    pub fn default() -> Self {
        // Open up the nounlist in read-only mode
        let filename = "nounlist.txt";
        let file = match File::open(filename) {
            Ok(filehandle) => filehandle,
            Err(_) => panic!("nounlist.txt file is broken or missing."),
        };

        let reader = BufReader::new(file);

        // TODO: 6801. is there any non stupid way to count the lines in the file?
        // apparently operating systems have word lists?

        let i = 6801;

        let num = rand::thread_rng().gen_range(0..=i);

        let word_select = match reader.lines().nth(num) {
            Some(Ok(wordt)) => wordt,
            Some(Err(_)) => panic!("Word selected from nounlist.txt is somehow unreadable."),
            None => panic!("There are no words in nounlist.txt."),
        };

        Word::create_word(word_select)
    }

    pub fn create_word(word: String) -> Self {
        let hidden: Vec<char> = word
            .chars()
            .map(|a| if a.is_ascii_alphabetic() { '_' } else { '-' })
            .collect();

        Word {
            revealed: word.to_uppercase(),
            hidden,
            length: word.len(),
        }
    }

    pub fn display_hidden(&self) -> String {
        list_with_spaces(&self.hidden)
    }

    pub fn check_guess(&mut self, player: &mut Player, guess: char) {
        // get the indices of the revealed word
        let indices: Vec<usize> = self
            .revealed
            .chars()
            .enumerate()
            .filter_map(|(idx, c)| if c == guess { Some(idx) } else { None })
            .collect();

        if indices.is_empty() {
            // Make hangman (player) HP decrement by 1, the gameloop itself will check hangman hp and display a relevant hangman
            println!("WRONG");
            player.hurt();
        } else {
            self.hidden.iter_mut().enumerate().for_each(|(i, c)| {
                if indices.contains(&i) {
                    *c = guess
                } else {
                }
            });
        }
    }

    pub fn show_answer(&self) -> String {
        self.revealed.clone()
    }
}
