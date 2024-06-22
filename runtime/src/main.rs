use std::io::{stdin, stdout, Write};
use skojarzenie::separate_text;
use skojarzenie::skoj::Skoj;

fn main() {
    let mut skoj = Skoj::new();

    loop {
        print!("[You]: ");
        stdout().flush().unwrap();

        let mut user_input = "".to_string();
        stdin().read_line(&mut user_input).unwrap();

        if &user_input == "exit\n" {
            break
        }

        for word in separate_text(user_input) {
            skoj.give_word(word);
        }

        print!("[Skoj]: ");
        for _ in 0..100 {
            let skoj_word = skoj.get_word().as_str();
            print!("{skoj_word}");

            if skoj_word == "." {
                break
            }
            if skoj_word != "," {
                print!(" ");
            }

        }
        println!();

    }

}