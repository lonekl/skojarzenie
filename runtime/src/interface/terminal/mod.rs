use std::io::{Error as IOError, stdin, stdout, Write};
use std::path::{Path, PathBuf};
use skojarzenie::separate_text;
use skojarzenie::skoj::Skoj;


pub struct TerminalProject {

    skoj: Skoj<String>,

}

impl TerminalProject {

    pub fn open(_workspace_path: &Path) -> Self {

        Self {
            skoj: Skoj::new(),
        }
    }

    pub fn enter(&mut self) -> TerminalProjectReturn {

        'runtime: loop {

            // Read your line
            let read_line = match read_line("[User]: ") {
                Ok(input) => input,
                Err(error) => return TerminalProjectReturn::ReadError(error),
            };

            if read_line.chars().next().map(|prefix| prefix == ':').unwrap_or(false) {

                // Dividing command into arguments.
                let mut arguments = vec![];
                let mut word_buffer = "".to_string();
                for line_char in read_line.chars().skip(1) {

                    if line_char.is_whitespace() {

                        if !word_buffer.is_empty() {

                            arguments.push(word_buffer);
                            word_buffer = "".to_string();
                        }
                    } else {

                        word_buffer.push(line_char);
                    }
                }

                if !word_buffer.is_empty() {

                    arguments.push(word_buffer);
                }

                // Running commands
                if arguments.is_empty() {

                    continue 'runtime
                }

                match arguments[0].as_str() {
                    "exit" => {
                        if arguments.len() > 1 {
                            println!("[Exit]: Please don't use arguments for that command.");
                        }
                        return TerminalProjectReturn::Close
                    },
                    _ => {
                        println!("[Terminal]: Command \"{}\" does not exist.", arguments[0]);
                    },
                }

            } else {

                // Giving chat bot your words.
                for word in separate_text(read_line) {
                    self.skoj.give_word(word);
                }

                // Giving you chat bots words.
                print!("[Skoj]: ");
                for _ in 0..100 {

                    let skoj_word = self.skoj.get_word().as_str();
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

    }

}

pub enum TerminalProjectReturn {

    ReadError(IOError),
    Enter(String),
    OpenNew(PathBuf),
    Close,

}



fn read_line(prompt: &str) -> Result<String, IOError> {
    stdout().write_all(prompt.as_bytes())?;
    stdout().flush()?;

    let mut input = "".to_string();
    stdin().read_line(&mut input)?;

    Ok(input)
}
