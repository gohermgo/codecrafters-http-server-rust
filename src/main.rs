use std::net;
#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};
#[allow(unused_imports)]
use std::sync::Arc;
#[macro_export]
macro_rules! log_from_mod_debug {
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
        println!("[{}] {}: {}", module_path!(), $msg, $val)
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
mod http;
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

                let mut stream_buffer = [0u8; GET_MAX_SIZE];
                let bytes_read = match stream.read(&mut stream_buffer) {
                    Ok(bytes_read) => {
                        log_from_mod!("bytes read", bytes_read);
                        bytes_read
                    }
                    Err(e) => {
                        elog_from_mod!("read failed", e);
                        0usize
                    }
                };
                // let buffer = stream_buffer[0.._n_read];
                let req = http::Request::try_construct(&stream_buffer, bytes_read).unwrap();
                // log_from_mod!("dumping request");
                // req.log();
                let res = http::Response::try_construct(req);
                log_from_mod!("attempting response", res);
                let _n_written = match stream.write(res.to_string().as_bytes()) {
                    Ok(bytes_written) => {
                        log_from_mod!("bytes written", bytes_written);
                        bytes_written
                    }
                    Err(e) => {
                        elog_from_mod!("write failed", e);
                        0usize
                    }
                };
            }
            Err(e) => elog_from_mod!("iffy error upon opening stream", e),
        })
}
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
