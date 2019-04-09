use std::fmt;

#[derive(Debug)]
pub struct Error(&'static str);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.0)
    }
}

impl std::error::Error for Error {}

impl Error {
    fn new(s: &'static str) -> Error {
        Error(s)
    }
}

impl From<&'static str> for Error {
    fn from(s: &'static str) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    fn return_error_result() -> Result<(), Box<std::error::Error>> {
        Err("".into())
    }

    #[test]
    fn test_return_error_result()  {
        assert!(return_error_result().is_err());
    }
}
