use {
    crate::http::{header::Error, Header},
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};
#[derive(Debug, Clone)]
/// Various types of user-agents
pub enum Kind {
    /// Curl user agent
    Curl(String),
    /// Any unrecognized/unimplemented user agent
    Unrecognized,
}
impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Kind::*;
        let user_agent_string = match self {
            Curl(version) => format!("curl/{}", version),
            Unrecognized => String::from("unrecognized"),
        };
        fmt::write(f, format_args!("{}", user_agent_string))
    }
}

const UNRECOGNIZED_USER_AGENT: Header = Header::UserAgent(Kind::Unrecognized);

impl FromStr for Kind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Kind::*;
        match s.split_once('/') {
            Some(("curl", version)) => Ok(Curl(version.to_string())),
            Some((agent, version)) => {
                let unrecognized_agent = format!("{}/{}", agent, version);
                Err(Error::Unrecognized(unrecognized_agent))
            }
            None => Err(Error::ParseFormat(UNRECOGNIZED_USER_AGENT, s.to_string())),
        }
    }
}
