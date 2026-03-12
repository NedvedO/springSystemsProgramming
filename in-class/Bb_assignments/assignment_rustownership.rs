fn most_frequent_word(text: &str) -> (String, usize) {
    // Split text into words (vector of string slices)
    let words: Vec<&str> = text.split_whitespace().collect();

    let mut max_word = "";
    let mut max_count = 0;

    // Outer loop: pick each word
    for i in 0..words.len() {
        let current_word = words[i];
        let mut count = 0;

        // Inner loop: count occurrences
        for j in 0..words.len() {
            if words[j] == current_word {
                count += 1;
            }
        }

        // Update max if needed
        if count > max_count {
            max_count = count;
            max_word = current_word;
        }
    }

    // Convert &str to String for returning (ownership transfer)
    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}