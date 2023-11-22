use std::net;
#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};
#[allow(unused_imports)]
use std::sync::Arc;
#[macro_export]
macro_rules! log_from_mod {
    // ( $msg:literal, $( $val:expr ),* ) => {
    //     let mut val_string = String::new();
    //     {
    //         for val in $val {

    //         }
    //     }
    //     for val in $val {}
    //     print!("{}\t{}", module_path!(), $msg, $val);
    //     println!("{}\t{}: {}", module_path!(), val[0], val);
    // };
    ($msg:literal, $val:expr) => {
        println!("[{}] {}: {:?}", module_path!(), $msg, $val)
    };
    ( $mgs:literal, $( $val:expr ),* ) => {
        print!("[{}] {}:", module_path!(), $msg);
        let x = $val;
        log_from_mod($msg, $val);
    };
    ($msg:expr, $val:expr) => {
        println!("[{}] {}: {}", module_path!(), $msg, $val)
    };
    ($msg:literal) => {
        println!("[{}] {}", module_path!(), $msg)
    };
}

#[macro_export]
macro_rules! elog_from_mod {
    ($msg:literal, $err:expr) => {
        eprintln!("[{}] {}: {}", module_path!(), $msg, $err)
    };
    ($msg:literal) => {
        eprintln!("[{}] {}", module_path!(), $msg)
    };
}

/// Module to store information regarding IPs
#[macro_use]
pub mod ip {
    use super::net;
    /// Module to store information specific to v4
    pub mod v4 {
        #[allow(unused_imports)]
        use super::net::{
            IpAddr::{self, V4},
            Ipv4Addr,
        };
        /// Submodule to store information regarding addresses
        #[macro_use]
        pub mod addr {
            #![allow(dead_code)]
            use super::*;

            pub const LOCAL_V4: Ipv4Addr = Ipv4Addr::LOCALHOST;
            pub const LOCAL_GENERIC: IpAddr = IpAddr::V4(LOCAL_V4);

            pub const EMPTY_V4: Ipv4Addr = Ipv4Addr::LOCALHOST;
            pub const EMPTY_GENERIC: IpAddr = IpAddr::V4(EMPTY_V4);

            pub const BROAD_V4: Ipv4Addr = Ipv4Addr::BROADCAST;
            pub const BROAD: IpAddr = IpAddr::V4(BROAD_V4);

            /// Submodule for utilities
            #[macro_use]
            pub mod utils {
                use super::*;

                /// Logs the address
                pub fn log(addr: &Ipv4Addr) {
                    log_from_mod!("address", addr);
                    // println!("[{}] address: {}", module_path!(), addr);
                }
                // pub fn _log(addr: &Ipv4Addr) {
                //     log_from_mod!("address", addr)
                // }
                // #[macro_export]
                // macro_rules! log {
                //     ($val:expr) => {
                //         (log_from_mod!("address", $val))
                //     }
                // }
                // pub use log;
            }
            // #[cfg(test)]
            // mod tests {
            //     use super::*;
            //     #[test]
            //     fn test_local_v4() {
            //         // TODO: handle port etc
            //         let ip = LOCAL_V4.octets();
            //         assert_eq!(ip, [127u8, 0u8, 0u8, 1u8]);
            //     }
            // }
        }
        /// Submodule to store information regarding to ports
        pub mod port {
            /// Given ip for the task
            pub const GIVEN: u16 = 4221;

            #[allow(dead_code)]
            pub mod utils {
                pub fn log(port: &u16) {
                    log_from_mod!("port", port);
                }
                // #[macro_export]
                // macro_rules! log_ {
                //     ($val:expr) => {
                //         (log_from_mod!("port", $val))
                //     }
                // }
                // pub use log;
            }
        }
    }
    /// Module to store information specific to v6
    mod v6 {
        // TODO: Implement
        // use super::net::{IpAddr::V6, Ipv6Addr};
        /// Submodule to store information regarding addresses
        /// Not yet implemented
        mod addr {
            // TODO: implement
        }
        /// Submodule to store information regarding ports
        /// Not yet implemented
        mod port {
            // TODO: implement
        }

        // Something more should be here for v6

        /// Submodule for utility function
        /// Not yet implemented
        mod utils {
            // TODO: implement
        }
    }
    /// Submodule for utility functions
    pub(super) mod utils {}
}

#[macro_use]
mod socket {
    use super::ip;
    pub mod v4 {
        use super::ip::v4;
        pub mod addr {
            #![allow(dead_code)]
            use super::*;
            use std::net::{SocketAddr, SocketAddrV4};

            pub const DEFAULT_V4: SocketAddrV4 =
                SocketAddrV4::new(v4::addr::LOCAL_V4, v4::port::GIVEN);

            pub const DEFAULT_GENERIC: SocketAddr = SocketAddr::V4(DEFAULT_V4);

            pub const FALLBACK_V4: SocketAddrV4 =
                SocketAddrV4::new(v4::addr::EMPTY_V4, v4::port::GIVEN);

            pub const FALLBACK_GENERIC: SocketAddr = SocketAddr::V4(FALLBACK_V4);
        }
        pub(super) mod utils {
            use std::net::SocketAddrV4;
            pub fn log(addrv4: &SocketAddrV4) {
                log_from_mod!("socket ip", addrv4.ip());
                log_from_mod!("socket port", &addrv4.port());
            }
        }
    }

    use super::SocketAddr::{self, V4, V6};
    pub fn log(socketaddr: &SocketAddr) {
        match socketaddr {
            V4(addrv4) => v4::utils::log(addrv4),
            V6(_a) => unimplemented!(),
        }
    }
}

mod tcp {
    #[allow(unused_imports)]
    use super::{socket, SocketAddr, TcpListener, TcpStream};
    /// Creates a `TcpListener` by attempting to bind to the
    /// passed `SocketAddr`.
    ///
    /// If the passed address can be bound to, the created
    /// `TcpListener` will attempt to bind to it.
    ///
    /// If the passed address cant be bound to, `TcpListener`
    /// will be bound to empty
    pub fn listener(socketaddr: SocketAddr) -> TcpListener {
        // println!("{} attempting to bind tcplistener", module_path!());

        // Log info regarding binding
        log_from_mod!("bind init");
        socket::log(&socketaddr);

        match socketaddr {
            SocketAddr::V4(socket_address) => match TcpListener::bind(socket_address) {
                Ok(tcplistener) => {
                    log_from_mod!("bind successful");
                    tcplistener
                }
                Err(e) => {
                    elog_from_mod!("bind failed", e);
                    elog_from_mod!("returning fallback tcplistener");
                    TcpListener::bind(crate::socket::v4::addr::FALLBACK_V4).unwrap()
                }
            },
            SocketAddr::V6(_socket_address) => unimplemented!(),
        }
    }

    // pub fn accept_on_thread(tcplistener: &TcpListener) -> (TcpS) {
    //     log_from_mod!("attempting to create thread for incoming streams")
    //     let local_addr = tcplistener.local_addr().unwrap();
    //     socket::log(&local_addr);
    //     log_from_mod!("resolved local address, starting")

    //     tcplistener.accept()
    // }
}

mod http {
    #[allow(unused_imports)]
    use std::{
        fmt::{Display, Write},
        path::PathBuf,
    };
    #[allow(dead_code)]
    const OK: &str = "HTTP/1.1 200 OK\r\n";
    const NOT_FOUND: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

    pub(super) enum Method {
        Get,
    }
    impl Method {
        pub fn log(&self) {
            match self {
                Method::Get => log_from_mod!("http method GET"),
            }
        }
    }
    pub mod header {
        #[derive(Debug)]
        pub enum Field {
            AcceptEncoding(String),
            ///
            ContentLength(usize),
            ContentType(String),
            /// Port may be omitted, if port is standard for the requested service
            Host(String, Option<u16>),
            /// The user agent string of the user agent
            /// Identifies who is responsible for making a given HTTP request
            UserAgent(String),
        }
        impl ToString for Field {
            fn to_string(&self) -> String {
                let header_string = match self {
                    Self::AcceptEncoding(encoding_type) => {
                        format!("Accept-Encoding: {}", encoding_type)
                    }
                    Self::ContentLength(content_length) => {
                        format!("Content-Length: {}", content_length)
                    }
                    Self::ContentType(content_type) => format!("Content-Type: {}", content_type),
                    Self::Host(host_name, port_opt) => {
                        let mut host_string = format!("Host: {}", host_name);
                        if let Some(port) = port_opt {
                            host_string = format!("{}:{}", host_string, port);
                        };
                        host_string
                    }
                    Self::UserAgent(user_agent) => format!("User-Agent: {}", user_agent),
                };
                format!("{}\r\n", header_string)
            }
        }
        pub fn construct(headers: Vec<Field>) -> String {
            headers
                .iter()
                .map(Field::to_string)
                .reduce(|acc, e| format!("{}{}", acc, e))
                .unwrap_or(String::new())
        }
        impl Field {
            pub fn try_parse(header_line: &str) -> Option<Self> {
                log_from_mod!("Trying to parse header field from", header_line);
                if let Some((key_string, value_string)) = header_line.split_once(':') {
                    // Header-Key: Header-Value
                    // let (key_string, value_string) = header_line.split_at(index);
                    // let (_, value_string) = value_string.split_at(1usize);
                    // Trim both
                    let (key_string, value_string) = (key_string.trim(), value_string.trim());
                    // Construct field based on key
                    match key_string {
                        "Accept-Encoding" => {
                            log_from_mod!("parsed header Accept-Encoding: ", value_string);
                            Some(Self::AcceptEncoding(String::from(value_string)))
                        }
                        "Content-Length" => {
                            if let Ok(octet_count) = value_string.parse::<usize>() {
                                log_from_mod!("parsed header Content-Length: ", octet_count);
                                Some(Self::ContentLength(octet_count))
                            } else {
                                elog_from_mod!(
                                    "failed to parse header Content-Length: ",
                                    value_string
                                );
                                None
                            }
                        }
                        "Content-Type" => {
                            log_from_mod!("parsed header Content-Type: ", value_string);
                            Some(Self::ContentType(String::from(value_string)))
                        }
                        "Host" => {
                            // Port may be omitted if port is standard for the requested service
                            let (host_string, port_number) =
                                if let Some((host_name, port)) = value_string.split_once(':') {
                                    log_from_mod!("host name", host_name);
                                    log_from_mod!("port num", port);
                                    (String::from(host_name), port.parse::<u16>().ok())
                                } else {
                                    (String::from(value_string), None)
                                };
                            // let (host_name, port) = if let Some(idx) = value_string.find(':') {
                            //     let (host_name, port) = value_string.split_once(idx);
                            //     (String::from(host_name), port.parse::<u16>().ok())
                            // } else {
                            //     (String::from(value_string), None)
                            // };
                            log_from_mod!("Parsed Host string: {}", host_string);
                            log_from_mod!("Parsed Host port: {}", port_number);
                            Some(Field::Host(host_string, port_number))
                        }
                        "User-Agent" => {
                            log_from_mod!("parsed header User-Agent: {}", value_string);
                            Some(Field::UserAgent(String::from(value_string)))
                        }
                        _ => {
                            elog_from_mod!("Unrecognized header key {}", key_string);
                            None
                        }
                    }
                } else {
                    elog_from_mod!("Failed to parse header line {}", header_line);
                    None
                }
            }
        }
    }
    #[allow(dead_code)]
    #[derive(Debug)]
    pub struct Header {
        key: String,
        value: String,
    }
    impl Display for Header {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}: {}", self.key, self.value)
        }
    }
    impl Header {
        pub fn new(header_string: &str) -> Option<Self> {
            if header_string.is_empty() {
                None
            } else {
                let _checking = header_string
                    .lines()
                    .map(header::Field::try_parse)
                    .for_each(|f| {
                        log_from_mod!("Heres something cool probably {}", f);
                    });
                let components = header_string
                    .split(':')
                    .map(|s| s.trim())
                    .collect::<Vec<&str>>();
                Some(Self {
                    key: components[0].to_string(),
                    value: components[1].to_string(),
                })
            }
        }
    }
    #[allow(dead_code)]
    pub struct Payload {
        method: Method,
        path: PathBuf,
        version: String,
        headers: Vec<header::Field>,
    }
    impl Payload {
        pub fn new(buffer: &[u8; super::GET_MAX_SIZE], bytes_read: usize) -> Self {
            let message = buffer[0..bytes_read]
                .iter()
                .map(|elem| *elem as char)
                .collect::<String>();
            // log_from_mod!("parsing message", message);
            log_from_mod!("parsing request");
            let mut message_components = message.lines();
            let start_line = message_components.nth(0).unwrap();
            log_from_mod!("start line", start_line);

            let mut start_line_iter = start_line.split_whitespace();

            let method_string = start_line_iter.nth(0).unwrap();
            log_from_mod!("method string", method_string);
            let method = match method_string {
                "GET" => Method::Get,
                _ => {
                    elog_from_mod!("weird method", method_string);
                    unimplemented!()
                }
            };

            // Successive calls to nth will remove this value
            let path_string = start_line_iter.nth(0).unwrap();
            log_from_mod!("path string", path_string);
            let path = PathBuf::from(path_string);

            let version = start_line_iter.nth(0).unwrap().to_string();
            log_from_mod!("version string", version);

            let headers = message_components
                .filter_map(header::Field::try_parse)
                .collect::<Vec<header::Field>>();
            // let headers: Vec<Header> = message_components.filter_map(Header::new).collect();
            headers
                .iter()
                .for_each(|header| log_from_mod!("request header", header));
            // let headers = message_components.nth(0).unwrap();
            // log_from_mod!("headers", headers);
            Self {
                method,
                path,
                version,
                headers,
            }
        }
        pub(super) fn dummy(headers: Vec<header::Field>) -> Self {
            Self {
                method: Method::Get,
                path: PathBuf::from("/"),
                version: String::from("HTTP/1.1"),
                headers,
            }
        }
        pub(crate) fn construct_headers(&self) -> String {
            // let headers = self.headers.iter().fold(String::new(), |mut output, elem| {
            // let _ = write!(output, "{}: {}\r\n", elem.key, elem.value);
            //     output
            // });
            // let mut header_field = String::new();
            let headers = self
                .headers
                .iter()
                .map(header::Field::to_string)
                .reduce(|acc, elem| format!("{}{}", acc, elem))
                .unwrap_or(String::new());
            // let header_field = self.headers.iter().fold(String::new())
            format!("{}\r\n", headers)
        }
        pub fn handle(&self) -> String {
            match self.method {
                Method::Get => match self.path.to_str().unwrap() {
                    "/" => format!("{}\r\n", OK),
                    path_string => {
                        let path_segments = path_string
                            .split('/')
                            .enumerate()
                            .filter_map(|(idx, e)| idx.ne(&0usize).then_some(e.to_string()))
                            .collect::<Vec<String>>();
                        // path_segments.remove(0usize);
                        // path_segments
                        //     .iter()
                        //     .for_each(|seg| log_from_mod!("segment", seg));

                        // let content_length =
                        //     path_segments.get(2).map_or(0u8, |msg| msg.len() as u8);
                        // let root_segment = path_segments[1];
                        match path_segments.get(0usize) {
                            Some(method) if method.eq(&String::from("echo")) => {
                                // let (_, message) = path_string.split_at("/echo/".len());
                                let message = path_segments
                                    .get(1..path_segments.len())
                                    .unwrap_or(&[])
                                    .to_vec()
                                    .join("/");
                                // .reduce(|acc, e| format!("{}{}", acc, e))
                                // .unwrap_or(String::new());
                                log_from_mod!("message is ", message);
                                // let message = path_segments[1..path_segments.len()]
                                //     .iter()
                                //     .reduce(|acc, e| format!("{}{}", acc, e).as_str())
                                //     .unwrap_or("");
                                let headers = header::construct(vec![
                                    header::Field::ContentType(String::from("text/plain")),
                                    header::Field::ContentLength(message.len()),
                                ]);

                                // let headers = format!(
                                //     "{}\r\n",
                                //     [
                                //         "Content-Type: text/plain",
                                //         format!("Content-Length: {}", message.len()).as_str(),
                                //     ]
                                //     .join("\r\n")
                                // );
                                // let constructed =
                                //     format!("{}{}\r\n{}", OK, headers.as_str(), message);
                                // println!("constructed headers", headers);
                                log_from_mod!("constructed as ", headers);
                                format!("{}{}{}", OK, headers, message)
                                // .concat()
                                // .as_str()
                                // let start_line = OK;
                                // let headers = vec![
                                //     "Content-Type: text/plain",
                                //     stringify!("Content-Length: {}", content_length),
                                // ]
                                // .join("\r\n")
                                // .as_str();
                            }
                            _ => NOT_FOUND.to_string(),
                        }
                    } // _ => NOT_FOUND,
                },
            }
        }
        #[allow(dead_code)]
        pub fn log(&self) {
            self.method.log();
            log_from_mod!("http path", self.path);
            log_from_mod!("http version", self.version);
        }
    }
}
const GET_MAX_SIZE: usize = 1024;

#[allow(unused_imports)]
use std::io::{Read, Write};
fn main() {
    log_from_mod!("entering main");
    // TODO: Make this more robust maybe CLI???
    // Make the listener

    let tcp_listener = tcp::listener(socket::v4::addr::DEFAULT_GENERIC);
    tcp_listener
        .incoming()
        .for_each(|tcp_stream| match tcp_stream {
            Ok(mut stream) => {
                log_from_mod!("new incoming connection");

                // log_from_mod!("attempting write");

                // let _n_written = match stream.write(HTTP_OK.as_bytes()) {
                //     Ok(bytes_written) => {
                //         log_from_mod!("bytes written", bytes_written);
                //         bytes_written
                //     }
                //     Err(e) => {
                //         elog_from_mod!("write failed", e);
                //         0usize
                //     }
                // };

                let mut stream_buffer = [0u8; GET_MAX_SIZE];
                let _n_read = match stream.read(&mut stream_buffer) {
                    Ok(bytes_read) => {
                        log_from_mod!("bytes read", bytes_read);
                        bytes_read
                        // let mut stream_buffer: Vec<u8> = vec![];
                        // log_from_mod!("attempting to read open stream");
                        // match stream.read(&mut stream_buffer) {
                        //     Ok(bytes_read) => {
                        //         log_from_mod!("got response, bytes read", bytes_read);
                        //         for byte_value in stream_buffer {
                        //             log_from_mod!("read byte", byte_value);
                        //         }
                        //     }
                        //     Err(e) => {
                        //         elog_from_mod!("error reading from previously valid stream", e)
                        //     }
                        // }
                    }
                    Err(e) => {
                        elog_from_mod!("read failed", e);
                        0usize
                    }
                };
                // let buffer = stream_buffer[0.._n_read];
                let req = http::Payload::new(&stream_buffer, _n_read);
                // log_from_mod!("dumping request");
                // req.log();
                let res = req.handle();
                log_from_mod!("attempting response ", res);
                let _n_written = match stream.write(res.as_bytes()) {
                    Ok(bytes_written) => {
                        log_from_mod!("bytes written", bytes_written);
                        bytes_written
                    }
                    Err(e) => {
                        elog_from_mod!("write failed", e);
                        0usize
                    }
                };
                // let req = http::Request::new(stream_buffer, _n_read);
                // let stream_data = stream_buffer[0.._n_read]
                //     .iter()
                //     .map(|elem| *elem as char)
                //     .collect::<String>();
                // let x = stream_buffer.map(|elem| elem as char);
                // for byte_data in stream_buffer {
                // log_from_mod!("byte received", byte_data);
                // }
                // log_from_mod!("received message", stream_data);
                // log_from_mod!("attempt to parse now...");
            }
            Err(e) => elog_from_mod!("iffy error upon opening stream", e),
        })
}
// open_streams.iter_mut().for_each(|stream| {
//     let mut stream_buffer: Vec<u8> = vec![];
//     log_from_mod!("attempting to read open stream");
//     match stream.read(&mut stream_buffer) {
//         Ok(bytes_read) => {
//             log_from_mod!("got response, bytes read", bytes_read);
//             for byte_value in stream_buffer {
//                 log_from_mod!("read byte", byte_value);
//             }
//         }
//         Err(e) => elog_from_mod!("error reading from previously valid stream", e),
//     }
// });
// for stream in tcp::listener(socket::v4::addr::DEFAULT_GENERIC).incoming() {
//     match stream {
//         Ok(mut s) => {
//             log_from_mod!("new connection");
//             match s.write(HTTP_OK.as_bytes()) {
//                 Ok(bytes_writen) => {
//                     log_from_mod!("wrote to connection, bytecount", bytes_writen)
//                 }
//                 Err(e) => elog_from_mod!("iffy error", e),
//             }
//         }
//         Err(e) => {
//             elog_from_mod!("iffy error", e);
//         }
//     }
// }
// #[cfg(test)]
// mod test {
//     mod http {
//         use crate::http::*;
//         #[test]
//         fn test_header_construction() {
//             let content_type = Header::new("Content-Type: text/plain").unwrap();
//             let content_length = Header::new("Content-Length: 3").unwrap();
//             let dummy_payload = Payload::dummy(vec![content_type, content_length]);
//             let headers_string = dummy_payload.construct_headers();
//             assert_eq!(
//                 "Content-Type: text/plain\r\nContent-Length: 3\r\n\r\n",
//                 headers_string.as_str()
//             )
//             // let dummy_payload = Payload {
//             //     method: Method::Get,
//             //     path: PathBuf::from("/"),
//             //     version: String::from("HTTP/1.1"),
//             //     headers: vec![content_type, content_length],
//             // };
//         }
//     }
// }
