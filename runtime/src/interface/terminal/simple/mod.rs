use std::io::{Error as IOError, stdin, stdout, Write};
use std::path::Path;
use skojarzenie::{Project, separate_text};
use skojarzenie::skoj::Skoj;
use crate::interface::terminal::commands::{COMMANDS, separate_arguments};
use crate::interface::terminal::TerminalProjectReturn;


pub struct TerminalProject {

    project: Project,

}

impl TerminalProject {

    pub fn open(_workspace_path: &Path) -> Self {

        Self {
            project: Project {
                skoj: Skoj::new(),
            },
        }
    }

    pub fn enter(&mut self) -> TerminalProjectReturn {

        // Read your line:
        let read_line = match read_line("[User]: ") {
            Ok(input) => input,
            Err(error) => return TerminalProjectReturn::ReadError(error),
        };

        if read_line.chars().next().map(|prefix| prefix == ':').unwrap_or(false) {

            // Dividing command into arguments:
            let arguments = separate_arguments(read_line.chars().skip(1));

            // Checking if command exists:
            if arguments.is_empty() {

                return TerminalProjectReturn::Continue
            }

            // Searching command:
            match COMMANDS.iter().find(|iterated_command| iterated_command.name_id == &arguments[0]) {
                None => {
                    eprintln!("[Terminal]: Command \"{}\" does not exist.", arguments[0]);
                },
                Some(command) => match command.pass_arguments(arguments.iter().skip(1).map(|argument| (*argument).clone()).collect()) {
                    Ok((flags, data)) => return (command.execute_code)(flags, data, &mut self.project),
                    Err(errors) => eprintln!("[{}]: Errors there: {errors:?}", command.name_id),
                },
            }

        } else {

            // Giving chat bot your words:
            for word in separate_text(read_line) {
                self.project.skoj.give_word(word);
            }

            // Giving you chat bot words:
            print!("[Skoj]: ");
            for _ in 0..100 {

                let skoj_word = self.project.skoj.get_word().as_str();
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

        TerminalProjectReturn::Continue
    }

}



fn read_line(prompt: &str) -> Result<String, IOError> {
    stdout().write_all(prompt.as_bytes())?;
    stdout().flush()?;

    let mut input = "".to_string();
    stdin().read_line(&mut input)?;

    Ok(input)
}
