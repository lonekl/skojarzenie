use skojarzenie::Project;
use crate::interface::terminal::TerminalProjectReturn;
use super::COMMANDS;



pub fn help(_flags: Vec<bool>, mut command_name_ids: Vec<String>, _: &mut Project) -> TerminalProjectReturn {

    if command_name_ids.is_empty() {

        command_name_ids.push("help".to_string());
    }

    let command_index_iterator = command_name_ids.iter()
        .map(|command_name_id| COMMANDS.iter()
            .find(|command_b| command_b.name_id == command_name_id)
        );

    let mut command_didnt_fail = true;
    let mut command_output = "".to_string();
    for (iterated_questioned_command_index, iterated_questioned_command) in command_index_iterator.enumerate() {
        match iterated_questioned_command {
            None => {
                command_didnt_fail = false;
                eprintln!("[help]: No such command as {}", command_name_ids[iterated_questioned_command_index]);
            },
            Some(iterated_command) => if command_didnt_fail {

                if command_name_ids.len() > 1 {

                    command_output.push_str(&format!("\n{}:\n\n", iterated_command.name_id))
                }

                command_output.push_str(&format!("Usage:\n\t{}\nInfo:\n\t{}\n", iterated_command.usage, iterated_command.info));

                if ! iterated_command.flags.is_empty() {

                    command_output.push_str("Options:\n");
                }

                for (flag, flag_short, flag_description) in iterated_command.flags {

                    command_output.push_str(&format!(
                        "\t--{flag}{}\t{flag_description}\n",
                        flag_short
                            .map(|flag_char| format!(" -{flag_char}"))
                            .unwrap_or("".to_string()),
                    ));
                }

            },
        }
    }

    if command_didnt_fail {

        print!("{command_output}");
    }

    TerminalProjectReturn::Continue
}