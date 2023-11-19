use std::net;
#[allow(unused_imports)]
use std::net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream, ToSocketAddrs};

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
    }

    use super::SocketAddr::{self, V4, V6};
    pub fn log(socketaddr: SocketAddr) {
        match socketaddr {
            V4(a) => {
                log_from_mod!("address", a.ip());
                // addr::utils::log(a.ip());
                // port::utils::log(&a.port());
                log_from_mod!("   port", &a.port());
            }
            V6(_a) => unimplemented!(),
        }
    }
}

mod tcp {
    use super::{SocketAddr, TcpListener};
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
        log_from_mod!("attempting to bind tcplistener");
        super::socket::log(socketaddr);
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
}
fn main() {
    log_from_mod!("entering main");
    // TODO: Make this more robust maybe CLI???
    let stream_buffers: Vec<Vec<u8>> = tcp::listener(socket::v4::addr::DEFAULT_GENERIC)
        .incoming()
        .filter_map(|tcpstream| match tcpstream {
            Ok(incoming_stream) => {
                // println!("{} accepted new connection", module_path!());
                log_from_mod!("tcplistener accepted new connection");
                Some(incoming_stream)
            }
            Err(e) => {
                elog_from_mod!("tcplistener failed to accept connection", e);
                elog_from_mod!("filtering...");
                // eprintln!("{} error accepting connection, error {}", module_path!(), e);
                None
            }
        })
        .map(|tcpstream| {
            let stream_peer_addr = tcpstream.peer_addr().unwrap();
            let stream_local_addr = tcpstream.local_addr().unwrap();

            log_from_mod!("stream peer address", stream_peer_addr);
            log_from_mod!("stream local address", stream_local_addr);
            // println!("{} attempting to read from [peer: {}] [local: {}]", module_path!(), stream_peer_addr, stream_local_addr);

            let mut tcpbuffer: Vec<u8> = vec![];

            if let Ok(bytes_returned) = tcpstream.peek(&mut tcpbuffer) {
                log_from_mod!("bytes read", bytes_returned);
                // log_from_mod!("buffer contents", tcpbuffer.iter().map(|byte| byte.as_char()).join(", "));
            } else {
                log_from_mod!("bytes read", 0usize);
            };

            tcpbuffer
        })
        .collect();
    stream_buffers
        .iter()
        .enumerate()
        .for_each(|(buffer_index, buffer_segment)| {
            log_from_mod!("dumping buffer data for buffer", buffer_index);
            buffer_segment
                .iter()
                .enumerate()
                .for_each(|(buffer_data_index, buffer_data_point)| {
                    log_from_mod!(
                        format!("data index {}", buffer_data_index),
                        buffer_data_point
                    );
                });
        });
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
