pub mod words;



pub fn separate_text(text: String) -> Vec<String> {
    let mut list = vec![];

    let mut last_char_index = 0;
    let mut word_buffer = "".to_string();
    for (char_index, char) in text.chars().into_iter().enumerate() {


        if !char.is_alphabetic() {

            list.push(word_buffer);
            word_buffer = "".to_string();

            last_char_index = char_index;

        }

        if char.is_whitespace() {
            last_char_index = char_index + 1;
        } else {
            word_buffer.push(char.to_ascii_lowercase());
        }


    }
    list.push(word_buffer);

    list
}
