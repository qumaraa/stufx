
use std::error;
use std::fmt;
use std::fmt::{Formatter, write};
use std::process;

#[derive(Debug)]
pub enum Error {
    FileNotFound,
    InvalidInput,
    Message(String),
    Custom(i32,String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileNotFound => write!(f,"FileNotFound ERROR"),
            Self::InvalidInput => write!(f,"InvalidInput ERROR"),
            Self::Message(msg) => write!(f,"{msg}"),
            Self::Custom(z,x) => {
                write!(f, "{x}").expect("expect()");
                process::exit(*z);
            }
        }
    }
}
impl error::Error for Error {}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
