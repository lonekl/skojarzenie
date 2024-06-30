use std::path::Path;
use crate::interface::terminal::{TerminalProject, TerminalProjectReturn};

pub mod interface;



fn main() {

    println!("You can type with your ai. To use commands, enter them after \':\', \":help\" for example.");

    let mut project = TerminalProject::open(Path::new("/"));

    'runtime: loop {
        match project.enter() {
            TerminalProjectReturn::ReadError(error) => panic!("{error}"),
            TerminalProjectReturn::Enter(_) => {},
            TerminalProjectReturn::OpenNew(_) => {},
            TerminalProjectReturn::Close => {
                break 'runtime
            },
        }
    }

}