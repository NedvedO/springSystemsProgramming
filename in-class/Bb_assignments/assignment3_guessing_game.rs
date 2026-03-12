fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    let secret: i32 = 17;

    let mut guess: i32 = 0;
    let mut guesses_taken: i32 = 0;

    loop {
        // Simulated strategy: move toward secret by +1 each time
        guess = guess + 1;
        guesses_taken = guesses_taken + 1;

        let result: i32 = check_guess(guess, secret);

        if result == 0 {
            println!("Guess {guess} was correct!");
            break;
        } else if result == 1 {
            println!("Guess {guess} was too high.");
        } else {
            println!("Guess {guess} was too low.");
        }
    }

    println!("It took {guesses_taken} guesses.");
}
