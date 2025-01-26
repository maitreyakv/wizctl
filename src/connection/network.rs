use derive_getters::Getters;
use std::{
    io,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread::sleep,
    time::{Duration, Instant},
};
use thiserror::Error;

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

pub fn init_socket() -> Result<UdpSocket, io::Error> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(bind_address)?;
    socket.set_nonblocking(true)?;
    Ok(socket)
}

pub fn broadcast_and_receive_datagrams(
    socket: &UdpSocket,
    broadcast_data: &Vec<u8>,
    port: u16,
) -> Result<Vec<Datagram>, NetworkError> {
    let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, port);
    socket.set_broadcast(true)?;
    socket.send_to(broadcast_data, broadcast_address)?;
    socket.set_broadcast(false)?;

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
            Err(e) => {
                if let NetworkError::IOError(ref io_error) = e {
                    if io_error.kind() == io::ErrorKind::WouldBlock {
                        break;
                    }
                }

                return Err(e);
            }
        }
    }

    Ok(datagrams)
}

pub fn send_and_receive_datagram(
    socket: &UdpSocket,
    send_data: &[u8],
    ip: &IpAddr,
    port: u16,
) -> Result<Datagram, NetworkError> {
    socket.send_to(send_data, SocketAddr::new(*ip, port))?;

    let max_wait_duration = Duration::from_secs(1);
    let start = Instant::now();
    loop {
        if start.elapsed() >= max_wait_duration {
            return Err(NetworkError::NoUdpResponse(max_wait_duration));
        }

        let datagram_result = recv_from_socket(socket);
        match datagram_result {
            Ok(datagram) => {
                if datagram.source_address().ip() != *ip {
                    return Err(NetworkError::IncorrectResponseAddress {
                        actual_address: datagram.source_address().ip(),
                        expected_address: *ip,
                    });
                }

                return Ok(datagram);
            }
            Err(e) => {
                if let NetworkError::IOError(ref io_error) = e {
                    if io_error.kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }
                }
                return Err(e);
            }
        };
    }
}

fn recv_from_socket(socket: &UdpSocket) -> Result<Datagram, NetworkError> {
    let mut buf = [0; 512];
    let (n_bytes, source_address) = socket.recv_from(&mut buf)?;
    if n_bytes == buf.len() {
        return Err(NetworkError::BufferTooSmall(n_bytes));
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
pub enum NetworkError {
    #[error("{0}")]
    IOError(#[from] io::Error),
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
