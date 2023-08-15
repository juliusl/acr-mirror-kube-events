use std::fmt::Display;


pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error; 

impl std::error::Error for Error {

}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO:
        write!(f, "error")
    }
}