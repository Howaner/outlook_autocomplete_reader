use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct InvalidPropertyTypeError(pub u16);
impl fmt::Display for InvalidPropertyTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Found not-known property type: 0x{:02x}", self.0)
    }
}
impl Error for InvalidPropertyTypeError {}

#[derive(Debug)]
pub struct MissingPropertyTagError(pub u32);
impl fmt::Display for MissingPropertyTagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing property tag: 0x{:02x}", self.0)
    }
}
impl Error for MissingPropertyTagError {}

#[derive(Debug)]
pub struct TooMuchDataError(pub String, pub usize, pub usize);
impl fmt::Display for TooMuchDataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Too much data to read in {} (received len: {}, max len: {})", self.0, self.1, self.2)
    }
}
impl Error for TooMuchDataError {}
