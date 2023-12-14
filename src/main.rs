#[allow(unused_imports)]
use std::io::{self, Write};
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
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        let s = format!("[{} {}] {}: {}\n", line!(), module_path!(), $msg, $val);
        let _ = std::io::Write::write_all(&mut handle, s.as_bytes());
    };
    ( $mgs:literal, $( $val:expr ),* ) => {
        print!("[{} {}] {}:", line!(), module_path!(), $msg);
        let x = $val;
        log_from_mod($msg, $val);
    };
    ($msg:expr, $val:expr) => {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        let s = format!("[{} {}] {}: {}\n", line!(), module_path!(), $msg, $val)
        let _ = std::io::Write::write_all(&mut handle, s.as_bytes());
    };
    ($msg:literal) => {
        println!("[{} {}] {}", line!(), module_path!(), $msg)
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
            }
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
        #[allow(dead_code)]
        pub(super) mod utils {
            use std::net::SocketAddrV4;
            pub fn log(addrv4: &SocketAddrV4) {
                log_from_mod!("socket ip", addrv4.ip());
                log_from_mod!("socket port", &addrv4.port());
            }
        }
    }

    use super::SocketAddr::{self, V4, V6};
    #[allow(dead_code)]
    pub fn log(socketaddr: &SocketAddr) {
        match socketaddr {
            V4(addrv4) => v4::utils::log(addrv4),
            V6(_a) => unimplemented!(),
        }
    }
}

mod tcp;

const GET_MAX_SIZE: usize = 1024;

#[allow(unused_imports)]
use std::io::Read;

fn handle_stream(tcp_stream: std::io::Result<TcpStream>) -> std::io::Result<()> {
    match tcp_stream {
        Ok(mut stream) => {
            log_from_mod!("new incoming connection");
            let mut stream_buffer = [0u8; GET_MAX_SIZE];

            let bytes_read = stream.read(&mut stream_buffer)?;

            let req = http::Request::try_construct(&stream_buffer, bytes_read).unwrap();

            let res = http::Response::try_from(req).unwrap();

            let _n_written = stream.write(res.to_string().as_bytes())?;
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn main() -> std::io::Result<()> {
    let tcp_listener = tcp::listener(socket::v4::addr::DEFAULT_GENERIC)?;
    for tcp_stream in tcp_listener.incoming() {
        std::thread::spawn(|| handle_stream(tcp_stream));
    }
    Ok(())
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
