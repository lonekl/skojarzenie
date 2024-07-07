use std::str::Chars;

pub const COMMANDS: () = ();

pub struct Command {

    pub name_id: &'static str,
    /// List of command acceptable flags. Example `["--verbose", Some('v')]`
    pub flags: &'static [(&'static str, Option<char>)],
    execute_code: fn(Vec<bool>, Vec<String>),

}

impl Command {

    pub fn execute(arguments: Vec<String>) {}

}



pub fn separate_arguments<I>(command_string: I) -> Vec<String>
where
    I: Iterator<Item = char>,
{

    let mut arguments = vec![];
    let mut word_buffer = "".to_string();
    for line_char in command_string {

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

    arguments
}
