use rand::Rng;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        let hidden: Vec<char> = word.chars().map(|a| if a.is_ascii_alphabetic() {'_'} else {'-'}).collect();

        Word {
            revealed: word.to_uppercase(),
            hidden: hidden,
            length: word.len(),
        }
    }

    pub fn check_guess(&mut self, guess: char) {


        let indices: Vec<usize> = self.revealed.chars()
                                                        .enumerate()
                                                        .filter_map(|(idx,c)| if c==guess {Some(idx)} else {None})
                                                        .collect();


        
        self.hidden.iter_mut()
                            .enumerate()
                            .for_each(|(i,c)| if indices.contains(&i) {*c=guess} else {});
                       

    }
}


