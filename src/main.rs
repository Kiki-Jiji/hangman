use std::io;
use std::cmp::Ordering;
use rand::Rng;
use std::fs;
use std::iter;

mod printMan;

pub struct Game {
    lives: i32,
    wordToGuess: String,
    lettersGuessed: String,
    hiddenWord: String,
    remaining_letters: String,
}

impl Game {
    pub fn atest(&self) {
        println!("test {}", self.lives.to_string());
    }

    pub fn remove_letters(&mut self, guess: &str){
    
        let mut hat = String::from("");
        
        for i in self.remaining_letters.chars(){
        
            if guess.contains(i) {
                hat.push('-')
            } else {
                hat.push(i)
            }
            
        }

        self.remaining_letters = String::from("");
        self.remaining_letters.push_str(&hat);
          
    }

}

fn main() {

    let guess_word = guessWord();

    //println!("{}", guess_word);

    let hashed_word = iter::repeat("*").take(guess_word.len()).collect::<String>();
    
    //println!("{}", hashed_word);

    let mut active_game = Game {
        lives: 6i32,
        wordToGuess: guess_word,
        lettersGuessed: String::from(""),
        hiddenWord: hashed_word,
        remaining_letters: String::from("a b c d e f g h i j k l m n o p q r s t u v w x y z"),
    };

    while active_game.lives >0 && active_game.lettersGuessed != active_game.hiddenWord {

        println!("You have {} lives left", active_game.lives);
        println!("{}", active_game.hiddenWord );
        printMan::print_hangman(active_game.lives);
        println!("Letters Remaining: {}", active_game.remaining_letters);

        // debug
        println!("words guessed debug, {}", active_game.lettersGuessed);

        let guess = guess_letter();
        
        active_game.remove_letters(&guess);

        // process the guess correct
        if active_game.wordToGuess.contains(&guess) {

            if active_game.lettersGuessed.contains(&guess){
                println!("You've already guessed this letter!");
            }

            println!("That letter is in the word!");
            active_game.lettersGuessed.push_str(&guess);

            active_game.hiddenWord = String::new();

            for i in active_game.wordToGuess.chars(){
                if active_game.lettersGuessed.contains(i){
                    active_game.hiddenWord.push(i)
                } else {
                    active_game.hiddenWord.push('-')
                }
            }

        } else if active_game.lettersGuessed.contains(&guess) {
            active_game.lettersGuessed.push_str(&guess);
            println!("You've aleady tried that letter silly :P ")

        } else  {
            active_game.lettersGuessed.push_str(&guess);
            println!("Your guess is not in the word boo");
            active_game.lives -= 1
        }

    } // end of while game loop

    if active_game.lettersGuessed == active_game.wordToGuess {
        println!("You guessed the word {}!! Congrats!!", active_game.wordToGuess);

    } else {
        printMan::print_hangman(active_game.lives);
        println!("fail :( The letter was, {}", active_game.wordToGuess)
    }

}

fn guessWord() -> String {

    let word_list = fs::read_to_string("word_list.txt").expect("Failed to read word list");

    let words = word_list.split("\n");
    
    let words_vec: Vec<&str> = words.collect();
    let mut rng = rand::thread_rng();
    let index_val = rng.gen_range(0, words_vec.len());

    let word_to_guess = words_vec[index_val];

    return word_to_guess.to_string();
}



fn guess_letter() -> String {

    loop {

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess2 = &guess.trim();

        let guess_length =  &guess2.chars().count();

        let char_size: usize = 1;

        match guess_length.cmp(&char_size) {
            Ordering::Less => println!("No character recongised"),
            Ordering::Greater => println!("Only one character please"),
            Ordering::Equal => {
                break guess2.to_string();
            }
            
        }

    }
}

