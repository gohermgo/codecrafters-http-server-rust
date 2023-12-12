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
    /// Go User Agent
    GoHttpClient(String),
    /// Any user agent
    Any(String),
    /// Unrecognized
    #[allow(dead_code)]
    Unrecognized,
}
impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Kind::*;
        let user_agent_string = match self {
            Curl(version) => format!("curl/{}", version),
            GoHttpClient(version) => format!("Go-http-client/{}", version),
            Any(ua) => ua.clone(),
            Unrecognized => String::from("unrecognized"),
        };
        fmt::write(f, format_args!("{}", user_agent_string))
    }
}

#[allow(dead_code)]
const UNRECOGNIZED_USER_AGENT: Header = Header::UserAgent(Kind::Unrecognized);

impl FromStr for Kind {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Kind::*;
        match s.split_once('/') {
            Some(("curl", version)) => Ok(Curl(version.to_string())),
            Some(("Go-http-client", version)) => Ok(GoHttpClient(version.to_string())),
            Some((_agent, _version)) => Ok(Any(s.to_string())),
            None => Ok(Any(s.to_string())),
        }
    }
}
