/// Utilities for UDP communications to devices
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread::sleep,
    time::Duration,
};

use derive_getters::Getters;
use error_stack::ResultExt;

#[derive(Debug, Getters)]
pub struct Datagram {
    data: Vec<u8>,
    source_address: SocketAddr,
}

pub fn broadcast_udp(
    broadcast_data: Vec<u8>,
    port: u16,
) -> error_stack::Result<Vec<Datagram>, io::Error> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket = UdpSocket::bind(bind_address).attach_printable("Could not bind socket!")?;
    socket
        .set_broadcast(true)
        .attach_printable("Could not set socket to broadcast!")?;
    socket
        .set_nonblocking(true)
        .attach_printable("Could not set socket to non-blocking!")?;

    let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, port);
    socket
        .send_to(&broadcast_data, broadcast_address)
        .attach_printable("Could not broadcast message!")?;

    sleep(Duration::from_secs(1));

    let mut datagrams = Vec::new();

    loop {
        let mut buf = [0; 4096];
        match socket.recv_from(&mut buf) {
            Ok((n_bytes, source_address)) => {
                // TODO: check for buffer overflow

                let data = buf[0..n_bytes].to_vec();
                if data == broadcast_data {
                    continue;
                }

                datagrams.push(Datagram {
                    data,
                    source_address,
                });
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                break;
            }
            Err(e) => return error_stack::Result::Err(error_stack::Report::new(e)),
        }
    }

    Ok(datagrams)
}
