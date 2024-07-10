use crate::skoj::Skoj;

pub mod skoj;



pub struct Project {

    pub skoj: Skoj<String>,

}



pub fn separate_text(text: String) -> Vec<String> {
    let mut list = vec![];

    let mut word_buffer = "".to_string();
    for char in text.chars() {


        if (!char.is_alphabetic() && !word_buffer.is_empty())
            ||
            word_buffer.chars().next().map(|buffer_char| !buffer_char.is_alphabetic()).unwrap_or(false) {

            list.push(word_buffer);
            word_buffer = "".to_string();

        }

        if !char.is_whitespace() {
            word_buffer.push(char.to_ascii_lowercase());
        }


    }
    if !word_buffer.is_empty() {
        list.push(word_buffer);
    }

    list
}
