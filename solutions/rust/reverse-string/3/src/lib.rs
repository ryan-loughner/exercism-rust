pub fn reverse(input: &str) -> String {

    // return early if input is an empty string
    if is_input_empty_string(input) {
        return String::new();
    }

    reverse_word(input)
}

fn is_input_empty_string(input: &str) -> bool {
    input.is_empty()
}

fn reverse_word(input: &str) -> String {
    let mut reversed_word = String::new();

    for char in input.chars().rev() {
        reversed_word.push(char);
    }
    reversed_word
}

// custom private functions
// if statements
// loops
// string functions