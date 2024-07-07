use crate::interface::terminal::TerminalProjectReturn;

pub const COMMANDS: &[Command] = &[
    Command {
        name_id: "exit",
        flags: &[
            ("help", None),
            ("say-goodbye", Some('g')),
        ],
        execute_code: |_, _| TerminalProjectReturn::Continue,
    },
    Command {
        name_id: "don't",
        flags: &[
            ("ujh", Some('u')),
            ("Polska", Some('P')),
            ("chwała", Some('ć')),
        ],
        execute_code: |_, _| TerminalProjectReturn::Continue,
    },
];

pub struct Command {

    pub name_id: &'static str,
    /// List of command acceptable flags. Example `["verbose", Some('v')]`.
    pub flags: &'static [(&'static str, Option<char>)],
    pub execute_code: fn(Vec<bool>, Vec<String>) -> TerminalProjectReturn,

}

impl Command {

    pub fn pass_arguments(&self, arguments: Vec<String>) -> Result<(Vec<bool>, Vec<String>), Vec<String>> {
        let mut flags = vec![false; self.flags.len()];
        let mut data = vec![];
        let mut errors = vec![];

        let mut no_more_flags = false;

        'argument_loop: for argument in arguments {

            if no_more_flags {

                data.push(argument);
                continue 'argument_loop
            }

            if argument.starts_with("--") {

                if &argument == "--" {

                    no_more_flags = true;
                    continue 'argument_loop
                }

                let flag: String = argument.chars().skip(2).collect();

                for (iterated_flag_index, (iterated_flag, _)) in self.flags.iter().enumerate() {

                    if &flag == *iterated_flag {

                        flags[iterated_flag_index] = true;
                        continue 'argument_loop
                    }
                }

                errors.push(argument);

            } else if argument.starts_with("-") {

                'flag_loop: for flag_short in argument.chars().skip(1) {
                    let flag_short_option = Some(flag_short);

                    for (iterated_flag_index, (_, iterated_flag)) in self.flags.iter().enumerate() {

                        if &flag_short_option == iterated_flag {

                            flags[iterated_flag_index] = true;
                            continue 'flag_loop
                        }
                    }

                    let mut error = "-".to_string();
                    error.push(flag_short);

                    errors.push(error);
                }

            } else {

                data.push(argument);
            }
        }

        if errors.len() > 0 {

            Err(errors)
        } else {

            Ok((flags, data))
        }
    }

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
