use crate::http::Header;
use std::fmt;

/// Errors related to headers
#[derive(Debug)]
pub enum Error {
    /// Errors related to parsing headers
    Parse(Header, String),
    /// Errors related to parsing format of headers
    #[allow(dead_code)]
    ParseFormat(Header, String),
    /// Errors related to unrecognized headers
    Unrecognized(String),
}
impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = module_path!();
        let body = match self {
            // Self::ParseUserAgent(message) => {
            // format!("{}: {}", "PARSE USER AGENT", message)
            // }
            Self::Parse(header_kind, message) => {
                format!("{} parse error: {}", header_kind.to_string(), message)
            }
            Self::ParseFormat(header_kind, message) => format!(
                "{} format parse error: {}",
                header_kind.to_string(),
                message
            ),
            Self::Unrecognized(message) => format!("unrecognized header {}", message),
        };
        fmt::write(f, format_args!("[{}] {}", source, body))
    }
}
