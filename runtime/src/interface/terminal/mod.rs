use std::path::PathBuf;
use std::io::Error as IOError;



pub mod simple;
pub mod commands;



pub enum TerminalProjectReturn {

    Continue,
    Enter(String),
    OpenNew(PathBuf),
    Close,
    ReadError(IOError),

}
