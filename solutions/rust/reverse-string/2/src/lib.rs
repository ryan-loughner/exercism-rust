pub fn reverse(input: &str) -> String {

    // return early if input is empty string
    if is_input_empty_string(input) {
        return "".to_string();
    }

    reverse_word(input)
}

fn is_input_empty_string(input: &str) -> bool {
    input.is_empty()
}

fn reverse_word(input: &str) -> String {
    let mut index = 0;
    let mut reversed_word = "".to_string();
    let input_length = input.len();
    let input_chars = input.chars().rev();

    for char in input_chars {
        reversed_word.push(char);
        index += 1;
        if index == input_length {
            break;
        }
    }
    reversed_word
}

// custom private functions
// if statements
// loops
// string functions
// arrays
