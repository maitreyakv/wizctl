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

#[derive(Debug, Getters)]
pub struct Datagram {
    data: Vec<u8>,
    source_address: SocketAddr,
}

#[derive(Error, Debug, Default)]
#[error("Could not complete UDP operation!")]
pub struct UdpError {}

pub fn broadcast_udp_and_receive_responses(
    broadcast_data: Vec<u8>,
    port: u16,
) -> error_stack::Result<Vec<Datagram>, UdpError> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, port);
    let socket = UdpSocket::bind(bind_address)
        .attach_printable("Could not bind socket!")
        .change_context(UdpError::default())?;
    socket
        .set_broadcast(true)
        .attach_printable("Could not set socket to broadcast!")
        .change_context(UdpError::default())?;
    socket
        .set_nonblocking(true)
        .attach_printable("Could not set socket to non-blocking!")
        .change_context(UdpError::default())?;

    let broadcast_address = SocketAddrV4::new(Ipv4Addr::BROADCAST, port);
    socket
        .send_to(&broadcast_data, broadcast_address)
        .attach_printable("Could not broadcast message!")
        .change_context(UdpError::default())?;

    sleep(Duration::from_secs(1));

    let mut datagrams = Vec::new();

    loop {
        let mut buf = [0; 256];
        match socket.recv_from(&mut buf) {
            Ok((n_bytes, source_address)) => {
                if n_bytes == buf.len() {
                    return error_stack::Result::Err(error_stack::Report::new(UdpError::default()))
                        .attach_printable("Received message was too large for buffer!");
                }

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
            Err(e) => {
                return error_stack::Result::Err(
                    error_stack::Report::new(e).change_context(UdpError::default()),
                )
            }
        }
    }

    Ok(datagrams)
}

pub fn send_udp_and_receive_response(
    send_data: Vec<u8>,
    send_address: SocketAddrV4,
) -> error_stack::Result<Datagram, UdpError> {
    let bind_address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, send_address.port());
    let socket = UdpSocket::bind(bind_address)
        .attach_printable("Could not bind socket!")
        .change_context(UdpError::default())?;

    socket
        .send_to(&send_data, send_address)
        .attach_printable("Could not send message!")
        .change_context(UdpError::default())?;

    let mut buf = [0; 128];
    let (n_bytes, source_address) = socket
        .recv_from(&mut buf)
        .attach_printable("Could not receive response message!")
        .change_context(UdpError::default())?;

    // TODO: Check if response came from expected address

    if n_bytes == buf.len() {
        return error_stack::Result::Err(error_stack::Report::new(UdpError::default()))
            .attach_printable("Received message was too large for buffer!");
    }

    let data = buf[0..n_bytes].to_vec();

    Ok(Datagram {
        data,
        source_address,
    })
}
