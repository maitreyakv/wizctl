use anyhow::Result;
use derive_getters::Getters;
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread::sleep,
    time::Duration,
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

pub fn init_socket(for_broadcast: bool, port: u16) -> Result<UdpSocket> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket = UdpSocket::bind(bind_address)?;
    if for_broadcast {
        socket.set_broadcast(true)?;
    }
    socket.set_nonblocking(true)?;
    Ok(socket)
}

pub fn broadcast_udp_and_receive_responses(
    socket: &UdpSocket,
    broadcast_data: &Vec<u8>,
    port: u16,
) -> Result<Vec<Datagram>> {
    let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, port);
    socket.send_to(&broadcast_data, broadcast_address)?;

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
}

//impl UdpClient {
//
//    pub fn send_udp_and_receive_response(
//        &self,
//        send_data: &[u8],
//        ip: &Ipv4Addr,
//    ) -> error_stack::Result<Datagram, NetworkError> {
//        self.socket
//            .send_to(send_data, SocketAddrV4::new(*ip, PORT))
//            .change_context(NetworkError::FailedUdpSend)?;
//
//        let max_wait_duration = Duration::from_secs(1);
//        let start = Instant::now();
//        loop {
//            if start.elapsed() >= max_wait_duration {
//                return error_stack::Result::Err(NetworkError::FailedUdpReceive.into())
//                    .attach_printable(format!(
//                        "Did not receive a response within {:?}",
//                        max_wait_duration
//                    ));
//            }
//
//            let datagram_result = self.recv_from_socket();
//            match datagram_result {
//                Ok(datagram) => {
//                    if datagram.source_address().ip() != *ip {
//                        return error_stack::Result::Err(
//                            NetworkError::ReceivedMessageFromUnexpectedSource.into(),
//                        )
//                        .attach_printable(format!("sent to {}", ip))
//                        .attach_printable(format!("received from {}", datagram.source_address()));
//                    }
//
//                    return Ok(datagram);
//                }
//                Err(r) => {
//                    if r.current_context().kind() == io::ErrorKind::WouldBlock {
//                        continue;
//                    }
//
//                    return Err(r).change_context(NetworkError::FailedUdpReceive);
//                }
//            };
//        }
//    }
//}
