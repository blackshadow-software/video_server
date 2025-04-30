use std::net::SocketAddr;

pub fn server_ip_address() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 8800))
}
