/// Module to handle User-Agent headers
pub mod user_agent {
    use {
        crate::error::{IError, ParseUserAgentError},
        std::{
            fmt::{self, Display, Formatter},
            str::FromStr,
        },
    };
    #[derive(Clone)]
    pub enum Kind {
        Curl(String),
    }
    impl Display for Kind {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            use Kind::*;
            let user_agent_string = match self {
                Curl(version) => format!("curl/{}", version),
            };
            fmt::write(f, format_args!("{}", user_agent_string))
        }
    }
    impl FromStr for Kind {
        type Err = ParseUserAgentError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use Kind::*;
            match s.split_once('/').unwrap_or(("", "")) {
                ("curl", version) => Ok(Curl(version.to_string())),
                (agent, version) => Err(ParseUserAgentError::new(format!(
                    "unrecognized agent {}/{}",
                    agent, version
                ))),
            }
        }
    }
}
/// Module to handle Connection headers
pub mod connection {
    use {
        crate::error::{IError, ParseConnectionError},
        std::{
            fmt::{self, Display, Formatter},
            str::FromStr,
        },
    };
    #[derive(Copy, Clone)]
    pub enum Kind {
        KeepAlive,
    }
    impl Display for Kind {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            use Kind::*;
            let connection_string = match self {
                KeepAlive => String::from("keep-alive"),
            };
            fmt::write(f, format_args!("{}", connection_string))
        }
    }
    impl FromStr for Kind {
        type Err = ParseConnectionError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            use Kind::*;
            match s {
                "keep-alive" => Ok(KeepAlive),
                other => Err(ParseConnectionError::new(format!(
                    "Unknown connection header value {}",
                    other
                ))),
            }
        }
    }
}
/// Module to handle Content-Type headers
pub mod content_type {
    use {
        crate::error::{IError, ParseContentTypeError},
        std::{
            fmt::{self, Display, Formatter},
            str::FromStr,
        },
    };
    /// Content types
    #[derive(Copy, Clone)]
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
        type Err = ParseContentTypeError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "text/plain" => Ok(Self::Plaintext),
                other => Err(ParseContentTypeError::new(other)),
            }
        }
    }
}
use {
    crate::error::{IError, ParseHeaderError},
    std::{
        fmt::{self, Display, Formatter},
        str::FromStr,
    },
};
/// Headers
#[derive(Clone)]
pub enum Kind {
    /// Request Header
    Host(String, Option<u16>),
    /// Request Header
    UserAgent(user_agent::Kind),
    /// Request Header
    Accept(String),
    /// Request Header
    AcceptLanguage(String),
    /// Request Header
    AcceptEncoding(String),
    /// General Header
    Connection(connection::Kind),
    /// General Header
    UpgradeInsecureRequests(u32),
    /// Representation Header
    ContentType(content_type::Kind),
    /// Representation Header
    ContentLength(usize),
}
impl Display for Kind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use Kind::*;
        let header_string = match self {
            // Request Headers
            Host(host, None) => format!("Host: {}", host),
            Host(host, Some(port)) => format!("Host: {}:{}", host, port),
            UserAgent(user_agent) => format!("User-Agent: {}", user_agent),
            Accept(accepted) => format!("Accept: {}", accepted),
            AcceptLanguage(accepted_language) => {
                format!("Accept-Language: {}", accepted_language)
            }
            AcceptEncoding(accepted_encoding) => {
                format!("Accept-Encoding: {}", accepted_encoding)
            }
            // General Headers
            Connection(connection) => format!("Connection: {}", connection),
            UpgradeInsecureRequests(count) => {
                format!("Upgrade-Insecure-Requests: {}", count)
            }
            // Representation Headers
            ContentType(content_type) => format!("Content-Type: {}", content_type),
            ContentLength(content_length) => {
                format!("Content-Length: {}", content_length)
            }
        };
        fmt::write(f, format_args!("{}\r\n", header_string))
    }
}
impl FromStr for Kind {
    type Err = ParseHeaderError;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        use Kind::*;
        let (key, value) = value.split_once(':').unwrap_or(("", ""));
        match (key.trim(), value.trim()) {
            // Request Headers
            ("Host", host_address) => match host_address.split_once(':') {
                Some((address, port_string)) => match port_string.parse::<u16>() {
                    Ok(port) => Ok(Host(address.to_string(), Some(port))),
                    Err(e) => Err(ParseHeaderError::new(
                        format!("{} error on trying to parse port {}", e, port_string).as_str(),
                    )),
                },
                None => Ok(Host(host_address.to_string(), None)),
            },
            ("User-Agent", user_agent_string) => {
                match user_agent::Kind::from_str(user_agent_string) {
                    Ok(user_agent_kind) => Ok(UserAgent(user_agent_kind)),
                    Err(e) => Err(ParseHeaderError::new(e)),
                }
            }
            ("Accept", accepted) => Ok(Accept(accepted.to_string())),
            ("Accept-Language", accepted_language) => {
                Ok(AcceptLanguage(accepted_language.to_string()))
            }
            ("Accept-Encoding", accepted_encoding) => {
                Ok(AcceptEncoding(accepted_encoding.to_string()))
            }
            // General Headers
            ("Connection", connection_string) => {
                match connection::Kind::from_str(connection_string) {
                    Ok(connection_kind) => Ok(Connection(connection_kind)),
                    Err(e) => Err(ParseHeaderError::new(
                        format!(
                            "{} on trying to parse connection kind {}",
                            e, connection_string
                        )
                        .as_str(),
                    )),
                }
            }
            ("Upgrade-Insecure-Requests", count_string) => match count_string.parse::<u32>() {
                Ok(count) => Ok(UpgradeInsecureRequests(count)),
                Err(e) => Err(ParseHeaderError::new(
                    format!("{} error on trying to parse UIR count: {}", e, count_string).as_str(),
                )),
            },
            // Representation Headers
            ("Content-Type", content_type_string) => {
                match content_type::Kind::from_str(content_type_string) {
                    Ok(content_kind) => Ok(ContentType(content_kind)),
                    Err(e) => Err(ParseHeaderError::new(
                        format!(
                            "{} error on trying to parse header {}",
                            e, content_type_string
                        )
                        .as_str(),
                    )),
                }
            }
            ("Content-Length", content_length_string) => {
                match content_length_string.parse::<usize>() {
                    Ok(content_length) => Ok(ContentLength(content_length)),
                    Err(e) => Err(ParseHeaderError::new(
                        format!(
                            "{} error on trying to parse content length {}",
                            e, content_length_string
                        )
                        .as_str(),
                    )),
                }
            }
            _ => Err(ParseHeaderError::new(format!(
                "unknown header {}: {}",
                key, value
            ))),
        }
    }
}
impl Kind {
    /// Returns true for headers that modify the request by specifying it further, by giving context, or by conditionally restricting it
    #[allow(dead_code)]
    pub fn is_request_header(&self) -> bool {
        use Kind::*;
        matches!(
            self,
            Host(_, _) | UserAgent(_) | Accept(_) | AcceptLanguage(_) | AcceptEncoding(_)
        )
    }
    /// Returns true for headers that apply to the message as a whole
    #[allow(dead_code)]
    pub fn is_general_header(&self) -> bool {
        use Kind::*;
        matches!(self, Connection(_) | UpgradeInsecureRequests(_))
    }
    /// Returns true for headers that describe the original format of the message data and any encoding applied (only present if the message has a body)
    #[allow(dead_code)]
    pub fn is_representation_header(&self) -> bool {
        use Kind::*;
        matches!(self, ContentType(_) | ContentLength(_))
    }
}
