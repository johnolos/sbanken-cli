use std::error::Error;
use std::fmt;
use termion::{color, style};

pub struct CliError {
    details: String,
    color: bool,
}

impl CliError {
    pub fn new(msg: &str, color: bool) -> CliError {
        CliError {
            details: msg.to_string(),
            color,
        }
    }
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Debug for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.color {
            write!(
                f,
                "{}{}{}",
                color::Fg(color::Red),
                self.details,
                style::Reset
            )
        } else {
            write!(f, "{}", self.details)
        }
    }
}
