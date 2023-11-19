use std::net;
#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};
#[allow(unused_imports)]
use std::sync::Arc;
// wow what a cool comment
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
        log_from_mod!("attempting to bind tcplistener");
        socket::log(&socketaddr);

        match socketaddr {
            SocketAddr::V4(socket_address) => match TcpListener::bind(socket_address) {
                Ok(tcplistener) => {
                    log_from_mod!("(V4) successful tcplistener bind");
                    tcplistener
                }
                Err(e) => {
                    elog_from_mod!("(V4) failed tcplistener bind, error {}", e);
                    elog_from_mod!("(V4) returning fallback tcplistener");
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

const HTTP_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";

use std::io::{Read, Write};
fn main() {
    log_from_mod!("entering main");
    // TODO: Make this more robust maybe CLI???
    // Make the listener

    let tcp_listener = tcp::listener(socket::v4::addr::DEFAULT_GENERIC);
    let mut open_streams: Vec<TcpStream> = tcp_listener
        .incoming()
        .filter_map(|tcp_stream| match tcp_stream {
            Ok(mut stream) => {
                log_from_mod!("new incoming connection");
                match stream.write(HTTP_OK.as_bytes()) {
                    Ok(bytes_written) => {
                        log_from_mod!("replied, bytes written", bytes_written);
                        Some(stream)
                    }
                    Err(e) => {
                        elog_from_mod!("iffy error upon writing", e);
                        None
                    }
                }
            }
            Err(e) => {
                elog_from_mod!("iffy error upon opening stream", e);
                None
            }
        })
        .collect();
    open_streams.iter_mut().for_each(|stream| {
        let mut stream_buffer: Vec<u8> = vec![];
        log_from_mod!("attempting to read open stream");
        match stream.read(&mut stream_buffer) {
            Ok(bytes_read) => {
                log_from_mod!("got response, bytes read", bytes_read);
                for byte_value in stream_buffer {
                    log_from_mod!("read byte", byte_value);
                }
            }
            Err(e) => elog_from_mod!("error reading from previously valid stream", e),
        }
    });
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
}
