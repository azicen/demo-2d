use std::error;
use std::fmt;
use std::string::String;

#[derive(Debug)]
pub struct Error {
    description: String, // 描述
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

impl Error {
    pub fn new(description: String) -> Self {
        Error { description }
    }

    pub fn to_string(&self) -> String {
        format!("Error {{ description: {:?} }}", self.description)
    }
}

#[cfg(test)]
#[test]
fn error_new() {
    let e_result = "Error { description: \"error\" }";

    let e = Error::new("error".to_string());
    let e_to_string = e.to_string();
    let e_format = format!("{:?}", e);
    assert_eq!(e_result, e_to_string);
    assert_eq!(e_to_string, e_format);
}
