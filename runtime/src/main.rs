use std::path::Path;
use crate::interface::terminal::{simple::TerminalProject, TerminalProjectReturn};



pub mod interface;



fn main() {

    println!("You can type with your ai. To use commands, enter them after \':\', \":help\" for example.");

    let mut project = TerminalProject::open(Path::new("/"));

    'runtime: loop {
        match project.enter() {
            TerminalProjectReturn::Continue => {},
            TerminalProjectReturn::Enter(_) => {},
            TerminalProjectReturn::OpenNew(_) => {},
            TerminalProjectReturn::Close => {
                break 'runtime
            },
            TerminalProjectReturn::ReadError(error) => panic!("{error}"),
        }
    }

}