/// Utilities for UDP communications to devices
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread::sleep,
    time::{Duration, Instant},
};
use thiserror::Error;

use derive_getters::Getters;
use error_stack::ResultExt;

const PORT: u16 = 38899;

pub fn rssi_to_signal_strength(rssi: i8) -> String {
    if rssi < -70 {
        "\u{2840} ".to_string()
    } else if rssi < -60 {
        "\u{28e0} ".to_string()
    } else if rssi < -50 {
        "\u{28e0}\u{2846}".to_string()
    } else {
        "\u{28e0}\u{28fe}".to_string()
    }
}

#[derive(Debug, Getters, Clone)]
pub struct Datagram {
    data: Vec<u8>,
    source_address: SocketAddr,
}

#[derive(Error, Debug)]
pub enum NetworkError {
    #[error("Could not complete setup socket for UDP!")]
    FailedUdpSocketSetup,
    #[error("Could not broadcast UDP message!")]
    FailedUdpBroadcast,
    #[error("Could not send UDP message!")]
    FailedUdpSend,
    #[error("Could not receive UDP message!")]
    FailedUdpReceive,
    #[error("Received message from unexpected address!")]
    ReceivedMessageFromUnexpectedSource,
}

pub struct UdpClient {
    socket: UdpSocket,
}

impl UdpClient {
    pub fn new(broadcast: bool) -> error_stack::Result<Self, NetworkError> {
        let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, PORT);
        let socket =
            UdpSocket::bind(bind_address).change_context(NetworkError::FailedUdpSocketSetup)?;
        if broadcast {
            socket
                .set_broadcast(true)
                .change_context(NetworkError::FailedUdpSocketSetup)?;
        }
        socket
            .set_nonblocking(true)
            .change_context(NetworkError::FailedUdpSocketSetup)?;

        Ok(Self { socket })
    }

    fn recv_from_socket(&self) -> error_stack::Result<Datagram, io::Error> {
        let mut buf = [0; 256];
        let (n_bytes, source_address) = self.socket.recv_from(&mut buf)?;
        if n_bytes == buf.len() {
            return error_stack::Result::Err(
                io::Error::new(
                    io::ErrorKind::OutOfMemory,
                    "Received message was too large for buffer!",
                )
                .into(),
            )
            .attach_printable(format!("buffer size is {}", buf.len()))
            .attach_printable(format!("received message size is {}", n_bytes));
        }
        let data = buf[0..n_bytes].to_vec();

        Ok(Datagram {
            data,
            source_address,
        })
    }

    pub fn broadcast_udp_and_receive_responses(
        &self,
        broadcast_data: Vec<u8>,
    ) -> error_stack::Result<Vec<Datagram>, NetworkError> {
        let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, PORT);
        self.socket
            .send_to(&broadcast_data, broadcast_address)
            .change_context(NetworkError::FailedUdpBroadcast)?;

        sleep(Duration::from_secs(1));

        let mut datagrams = Vec::new();

        loop {
            match self.recv_from_socket() {
                Ok(datagram) => {
                    if *datagram.data() == broadcast_data {
                        continue;
                    }

                    datagrams.push(datagram);
                }
                Err(ref r) if r.current_context().kind() == io::ErrorKind::WouldBlock => {
                    break;
                }
                Err(r) => {
                    return Err(r.change_context(NetworkError::FailedUdpReceive));
                }
            }
        }

        Ok(datagrams)
    }

    pub fn send_udp_and_receive_response(
        &self,
        send_data: Vec<u8>,
        ip: Ipv4Addr,
    ) -> error_stack::Result<Datagram, NetworkError> {
        self.socket
            .send_to(&send_data, SocketAddrV4::new(ip, PORT))
            .change_context(NetworkError::FailedUdpSend)?;

        let wait_duration = Duration::from_secs(1);
        let start = Instant::now();
        loop {
            if start.elapsed() >= wait_duration {
                return error_stack::Result::Err(NetworkError::FailedUdpReceive.into())
                    .attach_printable(format!(
                        "Did not receive a response within {:?}",
                        wait_duration
                    ));
            }

            let datagram_result = self.recv_from_socket();
            match datagram_result {
                Ok(datagram) => {
                    if datagram.source_address().ip() != ip {
                        return error_stack::Result::Err(
                            NetworkError::ReceivedMessageFromUnexpectedSource.into(),
                        )
                        .attach_printable(format!("sent to {}", ip))
                        .attach_printable(format!("received from {}", datagram.source_address()));
                    }

                    return Ok(datagram);
                }
                Err(r) => {
                    if r.current_context().kind() == io::ErrorKind::WouldBlock {
                        continue;
                    }

                    return Err(r).change_context(NetworkError::FailedUdpReceive);
                }
            };
        }
    }
}
