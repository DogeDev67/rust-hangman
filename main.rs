use std::io;

fn main() {
    let secret_word = "rustacean";
    let mut guessed_letters: Vec<char> = Vec::new();
    let mut num_guesses = 0;
    let max_guesses = 6;
    
    println!("Welcome to Hangman!");
    loop {
        println!("Secret word: {}", display_word(secret_word, &guessed_letters));
        println!("Guessed letters: {}", guessed_letters.iter().collect::<String>());
        println!("Number of incorrect guesses: {}", num_guesses);
        
        let guess = get_guess();
        if guessed_letters.contains(&guess) {
            println!("You already guessed that letter!");
            continue;
        }
        guessed_letters.push(guess);
        
        if !secret_word.contains(guess) {
            num_guesses += 1;
            if num_guesses == max_guesses {
                println!("Game over! The secret word was '{}'.", secret_word);
                break;
            }
            println!("Incorrect guess. Try again.");
        } else if has_won(secret_word, &guessed_letters) {
            println!("Congratulations! You won.");
            break;
        }
    }
}

fn display_word(word: &str, guessed_letters: &Vec<char>) -> String {
    word.chars().map(|c| if guessed_letters.contains(&c) { c } else { '_' }).collect()
}

fn has_won(word: &str, guessed_letters: &Vec<char>) -> bool {
    word.chars().all(|c| guessed_letters.contains(&c))
}

fn get_guess() -> char {
    loop {
        println!("Guess a letter:");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess = guess.trim().chars().next();
        if let Some(c) = guess {
            return c;
        }
        println!("Please enter a single letter.");
    }
}
