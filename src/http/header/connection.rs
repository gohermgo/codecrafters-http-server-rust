use {
    crate::http::header::Error,
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
/// Connection kinds
pub enum Kind {
    /// Connection keep alive
    KeepAlive,
    /// Any unrecognized or unimplemented connection kind
    Unrecognized,
}
impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Kind::*;
        let connection_string = match self {
            KeepAlive => String::from("keep-alive"),
            Unrecognized => String::from("unrecognized"),
        };
        fmt::write(f, format_args!("{}", connection_string))
    }
}
impl FromStr for Kind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Kind::*;
        match s {
            "keep-alive" => Ok(KeepAlive),
            other => {
                let unknown_connection = format!("unknown connection kind value {}", other);
                Err(Error::Unrecognized(unknown_connection))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Kind;
    #[test]
    fn keep_alive_from_str() {
        let s = "keep-alive";
        assert_eq!(Kind::from_str(s).unwrap(), Kind::KeepAlive)
    }
}
