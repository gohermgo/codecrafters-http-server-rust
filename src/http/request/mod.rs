use std::{
    fmt::{self, Display, Formatter},
    str::FromStr,
};
#[derive(Copy, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Head,
    Options,
}
impl Display for Method {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Method::*;
        let method_string = match self {
            Get => "GET",
            Post => "POST",
            Put => "PUT",
            Head => "HEAD",
            Options => "OPTIONS",
        };
        fmt::write(f, format_args!("{}", method_string))
    }
}
#[derive(Debug, Eq, PartialEq)]
pub struct ParseRequestMethodError(String);
impl Display for ParseRequestMethodError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::write(f, format_args!("{}", self.0))
    }
}
impl std::error::Error for ParseRequestMethodError {}
impl ParseRequestMethodError {
    fn new<S>(msg: S) -> Self
    where
        S: Display,
    {
        Self(format!("parse_request_method_error: {}", msg))
    }
}
impl FromStr for Method {
    type Err = ParseRequestMethodError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Method::*;
        match s {
            "GET" => Ok(Get),
            "POST" => Ok(Post),
            "PUT" => Ok(Put),
            "HEAD" => Ok(Head),
            "OPTIONS" => Ok(Options),
            _ => Err(ParseRequestMethodError::new(format!(
                "{} is not a recognized request method",
                s
            ))),
        }
    }
}
pub mod target {
    #[derive(Debug, Copy, Clone)]
    #[allow(dead_code)]
    pub enum Form {
        Origin,
        Absolute,
        Authority,
        Asterisk,
    }
}
#[derive(Clone)]
pub struct Target {
    pub path: String,
    pub form: target::Form,
}
/// The start-line contains three elements:
///   1. An HTTP `Method`, either a verb or a noun, that describes the action to be performed
///   2. The request target, usually a URL, or the absolute path of the protocol, port, and domain are usually characterized between different HTTP `Method`s. It can be:
///     - An absolute path, ultimately followed by a ? and query string. This is the most common form, known as the origin form.
///     - A complete URL, known as the absolute form, is mostuly used with `Method::Get` when connected to a proxy.
///     - The authority component of a URL, consisting of the domain name and optionally the port (prefixed by a :) is called the authority form. It is only used with `Method::Connect` when setting up an HTTP tunnel.
///     - The asterisk form, a simple asterisk is used with `Method::Options`, representing the server as a whole.
///   3. The HTTP version, which defines the structure of the remaining message, acting as an indicator of the expected version to use for the response
#[derive(Clone)]
pub struct Startline {
    pub method: Method,
    pub target: Target,
    pub version: super::Version,
}
impl Display for Startline {
    /// U should never have to make a startline for a request xd
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::write(
            f,
            format_args!(
                "{} {} {}\r\n\r\n",
                self.method, self.target.path, self.version
            ),
        )
    }
}
impl FromStr for Startline {
    type Err = std::io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split_whitespace();
        if components.clone().count() != 3 {
            let error = std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "component count longer not equal to 3",
            );
            Err(error)
        } else {
            let method = match components.nth(0usize).unwrap() {
                "GET" => Method::Get,
                "POST" => Method::Post,
                _ => unimplemented!(),
            };
            let target_component = components.nth(0usize).unwrap();
            let target = if target_component.contains('*') {
                Target {
                    path: "*".to_string(),
                    form: target::Form::Asterisk,
                }
            } else {
                Target {
                    path: target_component.to_string(),
                    form: target::Form::Absolute,
                }
            };
            let version = match components.nth(0usize).unwrap() {
                "HTTP/1.1" => super::Version(1u8, Some(1u8)),
                "HTTP/2" => super::Version(2u8, None),
                _ => unimplemented!(),
            };
            let line = Self {
                method,
                target,
                version,
            };
            Ok(line)
        }
    }
}
impl Startline {
    #[allow(dead_code)]
    pub(super) fn try_parse(start_line: &str) -> Option<Self> {
        let mut components = start_line.split_whitespace();
        if components.clone().count() != 3 {
            None
        } else {
            let method = match components.nth(0usize).unwrap() {
                "GET" => Method::Get,
                "POST" => Method::Post,
                _ => unimplemented!(),
            };
            let target_component = components.nth(0usize).unwrap();
            let target = if target_component.contains('*') {
                Target {
                    path: "*".to_string(),
                    form: target::Form::Asterisk,
                }
            } else {
                Target {
                    path: target_component.to_string(),
                    form: target::Form::Absolute,
                }
            };
            // let path =
            //  components
            //     .nth(0usize)
            //     .unwrap()
            //     .split('/')
            //     .filter(!String::is_empty)
            //     .collect::<Vec<String>>();
            let version = match components.nth(0usize).unwrap() {
                "HTTP/1.1" => super::Version(1u8, Some(1u8)),
                "HTTP/2" => super::Version(2u8, None),
                _ => unimplemented!(),
            };
            let line = Self {
                method,
                target,
                version,
            };
            Some(line)
        }
    }
}
