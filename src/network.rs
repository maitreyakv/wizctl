/// Utilities for UDP communications to devices
use std::{
    io,
    net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket},
    thread::sleep,
    time::Duration,
};
use thiserror::Error;

use derive_getters::Getters;
use error_stack::ResultExt;

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

fn setup_socket(bind_address: SocketAddrV4, broadcast: bool) -> io::Result<UdpSocket> {
    let socket = UdpSocket::bind(bind_address)?;
    if broadcast {
        socket.set_broadcast(true)?;
    }
    socket.set_nonblocking(true)?;
    Ok(socket)
}

fn recv_from_socket(socket: &UdpSocket) -> error_stack::Result<Datagram, io::Error> {
    let mut buf = [0; 256];
    let (n_bytes, source_address) = socket.recv_from(&mut buf)?;
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
    broadcast_data: Vec<u8>,
    port: u16,
) -> error_stack::Result<Vec<Datagram>, NetworkError> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket =
        setup_socket(bind_address, true).change_context(NetworkError::FailedUdpSocketSetup)?;

    let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, port);
    socket
        .send_to(&broadcast_data, broadcast_address)
        .change_context(NetworkError::FailedUdpBroadcast)?;

    sleep(Duration::from_secs(1));

    let mut datagrams = Vec::new();

    loop {
        match recv_from_socket(&socket) {
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
    send_data: Vec<u8>,
    send_address: SocketAddrV4,
) -> error_stack::Result<Datagram, NetworkError> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, send_address.port());
    let socket =
        setup_socket(bind_address, false).change_context(NetworkError::FailedUdpSocketSetup)?;

    socket
        .send_to(&send_data, send_address)
        .change_context(NetworkError::FailedUdpSend)?;

    let wait_duration = Duration::from_secs(1);
    sleep(wait_duration);

    let datagram = recv_from_socket(&socket)
        .map_err(|r| {
            if r.current_context().kind() == io::ErrorKind::WouldBlock {
                io::Error::new(
                    io::ErrorKind::TimedOut,
                    format!("Did not receive response after {:?}", wait_duration),
                )
                .into()
            } else {
                r
            }
        })
        .change_context(NetworkError::FailedUdpReceive)?;

    if *datagram.source_address() != send_address.into() {
        return error_stack::Result::Err(NetworkError::ReceivedMessageFromUnexpectedSource.into())
            .attach_printable(format!("sent to {}", send_address))
            .attach_printable(format!("received from {}", datagram.source_address()));
    }

    Ok(datagram)
}
