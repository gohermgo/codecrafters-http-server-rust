use std::fmt;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    ParseVersion(String, String),
    ParseVersionFormat(String),
}

impl std::error::Error for Error {}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let source = module_path!();
        let body = match self {
            Self::ParseVersion(message_1, message_2) => {
                format!("parse version error {}/{}", message_1, message_2)
            }
            Self::ParseVersionFormat(message) => format!("parse version format error {}", message),
        };
        fmt::write(f, format_args!("[{}] {}", source, body))
    }
}
