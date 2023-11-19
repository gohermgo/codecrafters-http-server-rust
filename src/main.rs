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

fn main() {
    log_from_mod!("entering main");
    // TODO: Make this more robust maybe CLI???
    // Make the listener

    let _ = tcp::listener(socket::v4::addr::DEFAULT_GENERIC)
        .incoming()
        .filter_map(|tcp_ping| match tcp_ping {
            Ok(tcp_stream) => {
                let mut stream_buffer = vec![];
                let bytes_received = tcp_stream
                    .peek(&mut stream_buffer)
                    .expect("couldnt peek da buffer");
                log_from_mod!("bytes received", bytes_received);
                log_from_mod!("local address");
                let local_addr = tcp_stream.local_addr().unwrap();
                socket::log(&local_addr);

                log_from_mod!("peer address");
                let peer_addr = tcp_stream.local_addr().unwrap();
                socket::log(&peer_addr);

                Some(tcp::listener(peer_addr))
            }
            Err(e) => {
                elog_from_mod!("something weird happened", e);
                elog_from_mod!("filtering that shit out");
                None
            }
        });
    // .collect();
    // tcp_incoming.iter().for_each(|tcp_stream| {
    //     let local_addr = tcp_stream.local_addr().unwrap();
    //     socket::log(&local_addr);
    // });
    // let stream_buffers: Vec<TcpStream> = tcp_listener
    //     .incoming()
    //     .filter_map(|tcpstream| match tcpstream {
    //         Ok(incoming_stream) => {
    //             // println!("{} accepted new connection", module_path!());
    //             log_from_mod!("");
    //             Some(incoming_stream)
    //         }
    //         Err(e) => {
    //             elog_from_mod!("filtering...");
    //             // eprintln!("{} error accepting connection, error {}", module_path!(), e);
    //             None
    //         }
    //     })
    //     .collect();
    // let tcp_listener = tcp::listener(socket::v4::addr::DEFAULT_GENERIC);
    // let stream_buffers: Vec<TcpStream> = tcp_listener
    //     .incoming()
    //     .filter_map(|tcpstream| match tcpstream {
    //         Ok(incoming_stream) => {
    //             // println!("{} accepted new connection", module_path!());
    //             log_from_mod!("");
    //             Some(incoming_stream)
    //         }
    //         Err(e) => {
    //             elog_from_mod!("tcplistener failed to accept connection", e);
    //             elog_from_mod!("filtering...");
    //             // eprintln!("{} error accepting connection, error {}", module_path!(), e);
    //             None
    //         }
    //     })
    //     .collect();
    // stream_buffers.iter().for_each(|incoming_stream| {

    // });
    // .map(|tcpstream| {
    //     let stream_peer_addr = tcpstream.peer_addr().unwrap();

    //     log_from_mod!("connection made from", stream_peer_addr);
    //     // println!("{} attempting to read from [peer: {}] [local: {}]", module_path!(), stream_peer_addr, stream_local_addr);

    //     let mut tcpbuffer: Vec<u8> = vec![];

    //     if let Ok(bytes_returned) = tcpstream.peek(&mut tcpbuffer) {
    //         log_from_mod!("bytes read", bytes_returned);
    //         // log_from_mod!("buffer contents", tcpbuffer.iter().map(|byte| byte.as_char()).join(", "));
    //     } else {
    //         log_from_mod!("bytes read", 0usize);
    //     };

    //     tcpbuffer
    // })
    // .collect();
    // stream_buffers
    //     .iter()
    //     .enumerate()
    //     .for_each(|(buffer_index, buffer_segment)| {
    //         log_from_mod!("dumping buffer data for buffer", buffer_index);
    //         buffer_segment
    //             .iter()
    //             .enumerate()
    //             .for_each(|(buffer_data_index, buffer_data_point)| {
    //                 log_from_mod!(
    //                     format!("data index {}", buffer_data_index),
    //                     buffer_data_point
    //                 );
    //             });
    //     });
    // let listener = tcp::listener(socket::v4::addr::DEFAULT_GENERIC);

    // let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    // for stream in listener.incoming() {
    // match stream {
    //     Ok(_stream) => {
    //         println!("accepted new connection");
    //     }
    //     Err(e) => {
    //         println!("error: {}", e);
    //     }
    // }
    // }
}
