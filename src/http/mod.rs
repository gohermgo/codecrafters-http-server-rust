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
    body: Vec<u8>,
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
        let request_lines = request_str
            .lines()
            .filter(|s| !s.is_empty())
            .collect::<Vec<&str>>();
        let start_line =
            request::Startline::from_str(request_lines.get(0usize).unwrap_or(&"")).ok();
        let headers = request_lines
            .iter()
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
        let header_count = headers.len();
        let body = request_lines
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                if i.ge(&header_count) {
                    Some(e.as_bytes().to_vec())
                } else {
                    None
                }
            })
            .flatten()
            .collect::<Vec<u8>>();
        if let Some(start_line) = start_line {
            let request = Request {
                start_line,
                headers,
                body,
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

// #[derive(Clone)]
// pub struct FilterMapI<I, F, A, B>
// where
//     I: Sized + Iterator<Item = (usize, A)>,
//     F: FnMut(I) -> Option<B>,
// {
//     iter: I,
//     f: F,
// }
// impl<I, F, A, B> FilterMapI<I, F, A, B> {
//     pub fn new(iter: I, f: F) -> FilterMapI<I, F, A, B>
//     where
//         I: Sized + Iterator<Item = (usize, A)>,
//         F: FnMut(I) -> Option<B>,
//     {
//         let iter = iter.enumerate();
//         return {};
//     }
// }

// impl<I: fmt::Debug, F, B: fmt::Debug> fmt::Debug for FilterMapI<I, F, B> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("FilterMap")
//             .field("iter", &self.iter)
//             .finish()
//     }
// }

pub trait IteratorExtensions {
    fn for_each_i<F>(self, f: F)
    where
        Self: Iterator + Sized,
        F: FnMut((usize, Self::Item)),
    {
        self.enumerate().for_each(f)
    }
    fn map_i<B, F>(self, f: F) -> std::iter::Map<std::iter::Enumerate<Self>, F>
    where
        Self: Iterator + Sized,
        F: FnMut((usize, Self::Item)) -> Option<B>,
    {
        self.enumerate().map(f)
    }
    fn filter_i<P>(self, predicate: P) -> std::iter::Filter<std::iter::Enumerate<Self>, P>
    where
        Self: Iterator + Sized,
        P: FnMut(&(usize, Self::Item)) -> bool,
    {
        self.enumerate().filter(predicate)
    }
    fn filter_map_i<B, F>(self, f: F) -> std::iter::FilterMap<std::iter::Enumerate<Self>, F>
    where
        Self: Iterator + Sized,
        F: FnMut((usize, Self::Item)) -> Option<B>,
    {
        self.enumerate().filter_map(f)
    }
}
impl<I: Iterator + Sized> IteratorExtensions for I {}
// impl<T, I: Iterator<Item = T>> IteratorExtensions for I
// where
//     I: Iterator + Sized,
// {
//     fn filter_map_i<B, F>(self, f: F) -> std::iter::FilterMap<std::iter::Enumerate<Self>, F>
//     where
//         F: FnMut((usize, Self::Item)) -> Option<B>,
//     {
//         self.enumerate().filter_map(f)
//     }
// }
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
        log_from_mod!("request startline", value.start_line);

        let version = value.start_line.version;
        let mut status = response::Status::NotFound;
        let mut headers: Vec<header::Kind> = vec![];
        let mut body = None;
        let request_path = value.start_line.target.path.clone();
        let request_path_components = request_path.split('/').into_iter().collect::<Vec<&str>>();
        let request_path_root = request_path_components.get(0usize);

        match (value.start_line.method, request_path_root) {
            (Get, _) if request_path.eq("/") => {
                status = response::Status::Ok;
                let start_line = response::Startline { version, status };
                Ok(Self {
                    start_line,
                    headers,
                    body,
                })
            }
            (Get, Some(&"echo")) => {
                let content = request_path_components
                    .into_iter()
                    .filter_map_i(|(i, e)| if i.ne(&0usize) { Some(e) } else { None })
                    // .enumerate()
                    // .filter_map(|(i, e)| if i.ne(&0usize) { Some(e) } else { None })
                    .collect::<Vec<&str>>()
                    .join("/");
                let content_length = content.len();
                headers.push(ContentType(Plaintext));
                headers.push(ContentLength(content_length));
                body = Some(content);
                status = response::Status::Ok;
                let start_line = response::Startline { version, status };
                Ok(Self {
                    start_line,
                    headers,
                    body,
                })
            }
            (Get, Some(&"user-agent")) => {
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
                let start_line = response::Startline { version, status };
                Ok(Self {
                    start_line,
                    headers,
                    body,
                })
            }
            (Get, Some(&"files")) => match std::env::args().nth(2usize) {
                Some(directory) => {
                    let content = request_path_components
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, e)| if i.ne(&0usize) { Some(e) } else { None })
                        .collect::<Vec<&str>>()
                        .join("/");
                    let file_string = vec![directory, content].join("/");
                    log_from_mod!("{}", file_string.clone());
                    let path = std::path::PathBuf::from(file_string);
                    if path.exists() {
                        let buf = std::fs::read(path.clone())?;
                        let buf_str = String::from_utf8(buf).unwrap_or_default();
                        log_from_mod!("{}", buf_str);
                        headers.push(ContentType(header::content_type::Kind::Appbytestream));
                        headers.push(ContentLength(buf_str.len()));
                        body = Some(buf_str);
                        status = response::Status::Ok;
                    } else {
                        log_from_mod!("path not found");
                        status = response::Status::NotFound;
                    };
                    let start_line = response::Startline { version, status };
                    Ok(Self {
                        start_line,
                        headers,
                        body,
                    })
                }
                None => {
                    panic!();
                }
            },
            (Get, _) => todo!(),
            (Post, Some(&"files")) => match std::env::args().nth(2usize) {
                Some(directory) => {
                    let content = request_path_components
                        .into_iter()
                        .filter_map_i(|(i, e)| if i.ne(&0usize) { Some(e) } else { None })
                        .collect::<Vec<&str>>()
                        .join("/");
                    let file_string = vec![directory, content].join("/");
                    let path = std::path::PathBuf::from(file_string);
                    headers.push(ContentType(header::content_type::Kind::Plaintext));
                    headers.push(ContentLength(value.body.len()));
                    std::fs::write(path, value.body)?;
                    let start_line = response::Startline { version, status };
                    Ok(Self {
                        start_line,
                        headers,
                        body,
                    })
                }
                None => {
                    panic!()
                }
            },
            (Post, _) => todo!(),
            (Put, _) => todo!(),
            (Options, _) => todo!(),
            (Head, _) => todo!(),
        }
    }
}
