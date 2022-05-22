use crate::list_with_spaces;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Player {
    hitpoints: u8,
    guess_list: HashSet<char>,
}

impl Player {
    pub fn default() -> Self {
        Player {
            hitpoints: 10,
            guess_list: HashSet::new(),
        }
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

    pub fn guessed_already(&self, guess: char) -> bool {
        self.guess_list.contains(&guess)
    }

    pub fn add_guess(&mut self, guess: char) {
        self.guess_list.insert(guess);
    }

    pub fn display_guesses(&self) -> String {
        let mut v = self.guess_list.clone().into_iter().collect::<Vec<char>>();
        v.sort_unstable();
        list_with_spaces(&v)
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
            println!("\x1b[93m{}\x1b[0m",line);
        }
    }
}
