use std::fmt::{self, Display, Formatter};
pub trait IError {
    fn new<S>(msg: S) -> Self
    where
        S: Display;
    fn info(&self) -> String;
    fn log_header() -> String {
        std::any::type_name::<Self>()
            .chars()
            .enumerate()
            .fold(vec![], |mut v, (i, c)| match i {
                0usize => {
                    v.push(c.to_ascii_lowercase());
                    v
                }
                _ => {
                    if c.is_uppercase() {
                        v.push('_');
                        v.push(c.to_ascii_lowercase());
                        v
                    } else {
                        v.push(c);
                        v
                    }
                }
            })
            .into_iter()
            .collect::<String>()
    }
}
/// Error for parsing user-agent header
#[derive(Debug, Eq, PartialEq)]
pub struct ParseUserAgentError(String);
impl std::error::Error for ParseUserAgentError {}
impl IError for ParseUserAgentError {
    fn new<S>(msg: S) -> Self
    where
        S: std::fmt::Display,
    {
        Self(msg.to_string())
    }
    fn info(&self) -> String {
        self.0.clone()
    }
}
impl Display for ParseUserAgentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::write(f, format_args!("{}: {}", Self::log_header(), self.info()))
    }
}
/// Error for parsing connection header
#[derive(Debug, Eq, PartialEq)]
pub struct ParseConnectionError(String);
impl std::error::Error for ParseConnectionError {}
impl IError for ParseConnectionError {
    fn new<S>(msg: S) -> Self
    where
        S: std::fmt::Display,
    {
        Self(msg.to_string())
    }
    fn info(&self) -> String {
        self.0.clone()
    }
}
impl Display for ParseConnectionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::write(f, format_args!("{}: {}", Self::log_header(), self.info()))
    }
}
/// Error for parsing content type header
#[derive(Debug, Eq, PartialEq)]
pub struct ParseContentTypeError(String);
impl std::error::Error for ParseContentTypeError {}
impl IError for ParseContentTypeError {
    fn new<S>(msg: S) -> Self
    where
        S: std::fmt::Display,
    {
        Self(msg.to_string())
    }
    fn info(&self) -> String {
        self.0.clone()
    }
}
impl Display for ParseContentTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::write(f, format_args!("{}: {}", Self::log_header(), self.info()))
    }
}
/// Error for parsing header in general
#[derive(Debug, Eq, PartialEq)]
pub struct ParseHeaderError(String);
impl std::error::Error for ParseHeaderError {}
impl IError for ParseHeaderError {
    fn new<S>(msg: S) -> Self
    where
        S: std::fmt::Display,
    {
        Self(msg.to_string())
    }
    fn info(&self) -> String {
        self.0.clone()
    }
}
impl Display for ParseHeaderError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::write(f, format_args!("{}: {}", Self::log_header(), self.info()))
    }
}
/// Error for parsing http version
#[derive(Debug, Eq, PartialEq)]
pub struct ParseHttpVersionError(String);
impl std::error::Error for ParseHttpVersionError {}
impl IError for ParseHttpVersionError {
    fn new<S>(msg: S) -> Self
    where
        S: Display,
    {
        Self(msg.to_string())
    }
    fn info(&self) -> String {
        self.0.clone()
    }
}
impl Display for ParseHttpVersionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::write(f, format_args!("{}: {}", Self::log_header(), self.info()))
    }
}
