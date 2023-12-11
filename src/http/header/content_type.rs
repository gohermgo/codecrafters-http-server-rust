use {
    crate::http::header::Error,
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};
/// Content types
#[derive(Debug, Copy, Clone)]
pub enum Kind {
    /// Plain text content
    Plaintext,
}
impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Kind::*;
        let content_type_string = match self {
            Plaintext => String::from("text/plain"),
        };
        fmt::write(f, format_args!("{}", content_type_string))
    }
}
impl FromStr for Kind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "text/plain" => Ok(Self::Plaintext),
            other => Err(Error::Unrecognized(other.to_string())),
        }
    }
}
