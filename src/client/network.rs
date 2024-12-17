use anyhow::Result;
use derive_getters::Getters;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread::sleep,
    time::{Duration, Instant},
};
use thiserror::Error;

//const PORT: u16 = 38899;
//
//pub fn rssi_to_signal_strength(rssi: i8) -> String {
//    if rssi < -70 {
//        "\u{2840} ".to_string()
//    } else if rssi < -60 {
//        "\u{28e0} ".to_string()
//    } else if rssi < -50 {
//        "\u{28e0}\u{2846}".to_string()
//    } else {
//        "\u{28e0}\u{28fe}".to_string()
//    }
//}

pub fn init_socket(port: u16) -> Result<UdpSocket> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket = UdpSocket::bind(bind_address)?;
    socket.set_nonblocking(true)?;
    Ok(socket)
}

pub fn broadcast_udp_and_receive_responses(
    socket: &UdpSocket,
    broadcast_data: &Vec<u8>,
    port: u16,
) -> Result<Vec<Datagram>> {
    let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, port);
    socket.send_to(broadcast_data, broadcast_address)?;

    sleep(Duration::from_secs(1));

    let mut datagrams = Vec::new();

    loop {
        match recv_from_socket(socket) {
            Ok(datagram) => {
                if datagram.data() == broadcast_data {
                    continue;
                }

                datagrams.push(datagram);
            }
            Err(e) => match e.downcast::<io::Error>() {
                Ok(io_error) => {
                    if io_error.kind() == io::ErrorKind::WouldBlock {
                        break;
                    }

                    return Err(io_error.into());
                }
                Err(other_error) => return Err(other_error),
            },
        }
    }

    Ok(datagrams)
}

pub fn send_udp_and_receive_response(
    socket: &UdpSocket,
    send_data: &[u8],
    ip: &IpAddr,
    port: u16,
) -> Result<Datagram> {
    socket.send_to(send_data, SocketAddr::new(*ip, port))?;

    let max_wait_duration = Duration::from_secs(1);
    let start = Instant::now();
    loop {
        if start.elapsed() >= max_wait_duration {
            return Err(NetworkError::NoUdpResponse(max_wait_duration).into());
        }

        let datagram_result = recv_from_socket(socket);
        match datagram_result {
            Ok(datagram) => {
                if datagram.source_address().ip() != *ip {
                    return Err(NetworkError::IncorrectResponseAddress {
                        actual_address: datagram.source_address().ip(),
                        expected_address: *ip,
                    }
                    .into());
                }

                return Ok(datagram);
            }
            // TODO: Deduplicate code
            Err(e) => match e.downcast::<io::Error>() {
                Ok(io_error) => {
                    if io_error.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }

                    return Err(io_error.into());
                }
                Err(other_error) => return Err(other_error),
            },
        };
    }
}

fn recv_from_socket(socket: &UdpSocket) -> Result<Datagram> {
    let mut buf = [0; 256];
    let (n_bytes, source_address) = socket.recv_from(&mut buf)?;
    if n_bytes == buf.len() {
        return Err(NetworkError::BufferTooSmall(n_bytes).into());
    }
    let data = buf[..n_bytes].to_vec();
    Ok(Datagram {
        data,
        source_address,
    })
}

#[derive(Debug, Getters)]
pub struct Datagram {
    data: Vec<u8>,
    source_address: SocketAddr,
}

#[derive(Debug, Error)]
enum NetworkError {
    #[error("received UDP message was too large for buffer of size {0}")]
    BufferTooSmall(usize),
    #[error("did not receive UDP response after {0:?}")]
    NoUdpResponse(Duration),
    #[error("received response from {actual_address}, but expected it from {expected_address}")]
    IncorrectResponseAddress {
        actual_address: IpAddr,
        expected_address: IpAddr,
    },
}
