use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Player {
    hitpoints: u8,
}

impl Player {
    pub fn default() -> Self {
        Player { hitpoints: 10 }
    }

    pub fn hurt(&mut self) {
        self.hitpoints -= 1;
    }

    pub fn kill(&mut self) {
        self.hitpoints = 0;
    }

    pub fn is_dead(&self) -> bool {
        self.hitpoints == 0
    }

    pub fn display_man(&self) {
        let mut m = [
            " _______".to_string(),    // 0
            "|      |".to_string(),    // 1
            "|      O".to_string(),    // 2
            "|     /|\\".to_string(),  // 3
            "|      |".to_string(),    // 4
            "|     / \\ ".to_string(), // 5
            "|".to_string(),           // 6
            "|____".to_string(),       // 7
        ];

        match self.hitpoints {
            10 => {
                for line in &mut m {
                    *line = String::new();
                }
            }
            9 => {
                for line in &mut m {
                    *line = String::new();
                }

                m[7] = " ____".to_string();
            }
            8 => {
                // quicker to modify the full hangman from this point
                m[0] = String::new();
                m.iter_mut()
                    .take(5 + 1)
                    .skip(1)
                    .for_each(|line| *line = "|".to_string()); // lines 1 to 5 become a stake
            }
            7 => {
                m.iter_mut()
                    .take(5 + 1)
                    .skip(1)
                    .for_each(|line| *line = "|".to_string()); // lines 1 to 5 become a stake
            }
            6 => {
                m.iter_mut()
                    .take(5 + 1)
                    .skip(2)
                    .for_each(|line| *line = "|".to_string()); // lines 2 to 5 become a stake
            }
            5 => {
                m.iter_mut()
                    .take(5 + 1)
                    .skip(3)
                    .for_each(|line| *line = "|".to_string()); // lines 3 to 5 become a stake
            }
            4 => {
                m[3] = "|      |".to_string();
                m[5] = "|".to_string();
            }
            3 => {
                m[3] = "|     /|".to_string();
                m[5] = "|".to_string();
            }
            2 => m[5] = "|".to_string(),
            1 => m[5] = "|     /  ".to_string(),
            _ => (), // leave full hangman ascii unmodified
        }

        for line in m {
            println!("{}", line);
        }
    }
}

#[derive(Debug)]
pub struct Word {
    revealed: String,
    pub hidden: Vec<char>,
    pub length: usize,
}

impl Word {
    pub fn default() -> Self {
        // Open up the nounlist in read-only mode
        let filename = "src/nounlist.txt";
        let file = match File::open(filename) {
            Ok(filehandle) => filehandle,
            Err(_) => panic!("Nounlist.txt is broken or missing!"),
        };

        let reader = BufReader::new(file);

        // 6801. is there any non-retarded way to count the lines in the file?
        // apparently operating systems have word lists?

        let i = 6801;

        let num = rand::thread_rng().gen_range(0..=i);

        let word_select = match reader.lines().nth(num) {
            Some(Ok(wordt)) => wordt,
            Some(Err(_)) => panic!("Word selected is somehow unreadable i dunno y."),
            None => panic!("Ain't no word here"),
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
        // put some spaces between every underscore to make it more readable
        // create the spaces then zip them with the hidden underscores
        let spaces = vec![' '; self.hidden.len()];

        self.hidden
            .iter()
            .zip(spaces)
            .map(|(c, space)| format!("{}{}", c, space))
            .collect::<String>()
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
            // Make hangman (player) HP decrement by 1, something somewhere else will check hangman hp and display a relevant hangman
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
