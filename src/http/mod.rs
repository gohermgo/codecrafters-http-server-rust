use std::ffi::OsString;

mod error;
pub(crate) mod header;
pub(crate) mod request;
pub(crate) mod response;

#[allow(unused_imports)]
use {
    error::Error,
    std::{
        fmt::{self, Display, Formatter, Write},
        path::PathBuf,
        str::FromStr,
    },
};
#[allow(dead_code)]
const OK: &str = "HTTP/1.1 200 OK\r\n";
#[allow(dead_code)]
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const MAX_BUFFER_SIZE: usize = 1024;

/// Struct to handle HTTP version
#[derive(Copy, Clone, Debug)]
pub struct Version(u8, Option<u8>);
impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.1 {
            Some(1u8) => fmt::write(f, format_args!("HTTP/1.1")),
            None => fmt::write(f, format_args!("HTTP/2")),
            _ => todo!(),
        }
    }
}
impl FromStr for Version {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once('/') {
            Some(("HTTP", version)) => {
                let (major, minor) = match version.split_once('.') {
                    Some((major_string, minor_string)) => (
                        major_string.parse::<u8>().unwrap(),
                        minor_string.parse::<u8>().ok(),
                    ),
                    _ => (version.parse::<u8>().unwrap(), None),
                };
                Ok(Self(major, minor))
            }
            Some((first, secnd)) => Err(Error::ParseVersion(first.to_string(), secnd.to_string())),
            _ => Err(Error::ParseVersionFormat(s.to_string())),
        }
    }
}
/// Typedef for Headers
pub type Header = crate::http::header::Kind;

/// HTTP requests are messages sent by the client to initiate an action on the server
#[allow(dead_code)]
pub struct Request {
    start_line: request::Startline,
    headers: Vec<Header>,
}
impl Request {
    pub fn try_construct(
        request_buffer: &[u8; MAX_BUFFER_SIZE],
        bytes_read: usize,
    ) -> Option<Self> {
        let request_str = request_buffer[0..bytes_read]
            .iter()
            .map(|x| *x as char)
            .collect::<String>();
        let mut request_lines = request_str.lines().filter(|s| !s.is_empty());
        let start_line = if let Some(start_line) = request_lines.nth(0) {
            request::Startline::try_parse(start_line)
        } else {
            None
        };
        let headers = request_lines
            .filter_map(
                |request_line| match str::parse::<header::Kind>(request_line) {
                    Ok(h) => Some(h),
                    Err(e) => {
                        elog_from_mod!("{}", e);
                        None
                    }
                },
            )
            .collect::<Vec<header::Kind>>();
        if let Some(start_line) = start_line {
            let request = Request {
                start_line,
                headers,
            };
            Some(request)
        } else {
            None
        }
    }
}
impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let headers = self
            .headers
            .clone()
            .into_iter()
            .fold(String::new(), |acc, elem| format!("{}{}", acc, elem));
        match self.body.clone() {
            Some(content) => std::fmt::write(
                f,
                format_args!("{}{}\r\n{}", self.start_line, headers, content),
            ),
            None => std::fmt::write(f, format_args!("{}{}\r\n", self.start_line, headers)),
        }
    }
}
pub struct Response {
    start_line: response::Startline,
    headers: Vec<header::Kind>,
    body: Option<String>,
}
impl TryFrom<Request> for Response {
    type Error = std::io::Error;
    fn try_from(value: Request) -> Result<Self, Self::Error> {
        use header::{content_type::Kind::*, Kind::*};
        use request::Method::*;
        match value.start_line.method {
            Get => {
                let version = value.start_line.version;
                let mut status = response::Status::NotFound;
                let mut headers: Vec<header::Kind> = vec![];
                let mut body = None;
                log_from_mod!("request startline", value.start_line);
                let request_path = value.start_line.target.path.clone();
                if request_path.eq("/") {
                    status = response::Status::Ok;
                    let start_line = response::Startline { version, status };
                    Ok(Self {
                        start_line,
                        headers,
                        body,
                    })
                } else {
                    let request_components = request_path
                        .split('/')
                        .filter(|s| !s.is_empty())
                        .collect::<Vec<&str>>();
                    match request_components.first() {
                        Some(root) if root.eq(&String::from("echo")) => {
                            let content = request_components
                                .into_iter()
                                .enumerate()
                                .filter_map(|(i, e)| if i.ne(&0usize) { Some(e) } else { None })
                                .collect::<Vec<&str>>()
                                .join("/");
                            let content_length = content.len();
                            headers.push(ContentType(Plaintext));
                            headers.push(ContentLength(content_length));
                            body = Some(content);
                            status = response::Status::Ok;
                        }
                        Some(root) if root.eq(&String::from("user-agent")) => {
                            let content = value
                                .headers
                                .iter()
                                .filter_map(|x| match x {
                                    Header::UserAgent(user_agent) => Some(user_agent.to_string()),
                                    _ => None,
                                })
                                .nth(0usize)
                                .unwrap();
                            headers.push(ContentType(Plaintext));
                            headers.push(ContentLength(content.len()));
                            body = Some(content);
                            status = response::Status::Ok;
                        }
                        Some(root) if root.eq(&String::from("files")) => {
                            let directory = std::env::args().nth(0usize).unwrap_or("/".to_string());
                            log_from_mod!("got directory {}", directory);
                            let content = request_components
                                .into_iter()
                                .enumerate()
                                .filter_map(|(i, e)| if i.ne(&0usize) { Some(e) } else { None })
                                .collect::<Vec<&str>>()
                                .join("/");
                            let path = std::path::PathBuf::from(directory);
                            let md = std::fs::metadata(path.clone())?;
                            if md.is_dir() {
                                log_from_mod!("is dir");
                            } else if md.is_file() {
                                log_from_mod!("is file");
                            } else if md.is_symlink() {
                                log_from_mod!("is symlink");
                            } else {
                                log_from_mod!("not sure")
                            }
                            let search = std::fs::read_dir(path.clone())?;
                            for hit in search {
                                match hit {
                                    Ok(somethign) => log_from_mod!(
                                        "{}",
                                        somethign.file_name().to_str().unwrap_or_default()
                                    ),
                                    Err(e) => elog_from_mod!("{}", e),
                                }
                            }
                            match std::fs::read_dir(path) {
                                Ok(mut directory_content) => {
                                    match directory_content.find(|x| match x {
                                        Ok(dir_entry) => dir_entry
                                            .file_name()
                                            .eq(&OsString::from(content.clone())),
                                        _ => false,
                                    }) {
                                        Some(file) => {
                                            let file_content = std::fs::read(file.unwrap().path())
                                                .unwrap_or_default();
                                            headers.push(ContentType(Plaintext));
                                            headers.push(ContentLength(file_content.len()));
                                            body = Some(
                                                String::from_utf8(file_content).unwrap_or_default(),
                                            );
                                            status = response::Status::Ok;
                                        }
                                        None => {
                                            status = response::Status::NotFound;
                                        }
                                    }
                                }
                                Err(_e) => {
                                    elog_from_mod!("{}", _e);
                                    status = response::Status::NotFound;
                                }
                            };
                        }
                        _ => (),
                    }
                    let start_line = response::Startline { version, status };
                    Ok(Self {
                        start_line,
                        headers,
                        body,
                    })
                }
            }
            Post => todo!(),
            Put => todo!(),
            Options => todo!(),
            Head => todo!(),
        }
    }
}
