use std::net::{SocketAddr, TcpListener};
/// Creates a `TcpListener` by attempting to bind to the
/// passed `SocketAddr`.
///
/// If the passed address can be bound to, the created
/// `TcpListener` will attempt to bind to it.
///
pub fn listener(socketaddr: SocketAddr) -> std::io::Result<TcpListener> {
    match socketaddr {
        SocketAddr::V4(socket_address) => TcpListener::bind(socket_address),
        SocketAddr::V6(_socket_address) => unimplemented!(),
    }
}
