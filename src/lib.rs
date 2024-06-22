pub mod skoj;



pub fn separate_text(text: String) -> Vec<String> {
    let mut list = vec![];

    let mut word_buffer = "".to_string();
    for char in text.chars() {


        if !char.is_alphabetic() {

            list.push(word_buffer);
            word_buffer = "".to_string();

        }

        if !char.is_whitespace() {
            word_buffer.push(char.to_ascii_lowercase());
        }


    }
    list.push(word_buffer);

    list
}
