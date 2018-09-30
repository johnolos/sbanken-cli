use chrono;
use regex;
use reqwest;
use std::convert;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    ArgumentMissing(&'static str),
    Regex(regex::Error),
    NoAccountFound(&'static str),
    EnvMissing(&'static str),
    Parsable(&'static str),
    Message(&'static str),
}

impl error::Error for Error {
    fn description(&self) -> &str {
        use self::*;

        match *self {
            Error::Reqwest(ref err) => err.description(),
            Error::ArgumentMissing(ref err) => err,
            Error::Regex(ref err) => err.description(),
            Error::NoAccountFound(ref err) => err,
            Error::EnvMissing(ref err) => err,
            Error::Parsable(ref err) => err,
            Error::Message(ref err) => err,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        use self::*;

        match *self {
            Error::Reqwest(ref err) => Some(err),
            Error::Regex(ref err) => Some(err),
            Error::ArgumentMissing(ref _err) => None,
            Error::NoAccountFound(ref _err) => None,
            Error::EnvMissing(ref _err) => None,
            Error::Parsable(ref _err) => None,
            Error::Message(ref _err) => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::*;

        match *self {
            Error::Reqwest(ref err) => write!(f, "Reqwest error: {}", err),
            Error::Regex(ref err) => write!(f, "Regex error: {}", err),
            Error::ArgumentMissing(ref err) => write!(f, "Arg missing error: {}", err),
            Error::NoAccountFound(ref err) => write!(f, "No account found: {}", err),
            Error::EnvMissing(ref err) => write!(f, "Error: env {} missing", err),
            Error::Parsable(ref err) => write!(f, "Parse error: {}", err),
            Error::Message(ref err) => write!(f, "Error: {}", err),
        }
    }
}

impl convert::From<chrono::ParseError> for Error {
    fn from(_err: chrono::ParseError) -> Self {
        Error::Message("Couldn't parse date on format `YYYY-MM-DD`")
    }
}

impl convert::From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl convert::From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::Regex(err)
    }
}
