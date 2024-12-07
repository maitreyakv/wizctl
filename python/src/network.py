"""Automatic discovery of available lights on the network"""

import logging
import socket
import time
from collections.abc import Generator

__logger__ = logging.getLogger(__name__)

PORT = 38899
BROADCAST_IP = "255.255.255.255"

MAX_EXPECTED_MESSAGE_SIZE = 4096


def _initialize_socket_for_broadcast(port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    sock.bind(("", port))
    sock.setblocking(False)
    return sock


def broadcast_udp(
    broadcast_data: bytes,
    wait_time: int,
) -> Generator[tuple[bytes, tuple[str, int]], None, None]:
    try:
        sock = _initialize_socket_for_broadcast(PORT)
        __logger__.debug(f"initialized {sock}")

        sock.sendto(broadcast_data, (BROADCAST_IP, PORT))
        time.sleep(wait_time)

        while True:
            try:
                received_data, (ip_addr, port) = sock.recvfrom(MAX_EXPECTED_MESSAGE_SIZE)
                __logger__.debug(f"Received message from {ip_addr}:{port} - {received_data!r}")

                # Ignore the self-broadcasted message
                if received_data == broadcast_data:
                    continue

                yield received_data, (ip_addr, port)

            except BlockingIOError:
                break
    finally:
        sock.close()
        __logger__.debug(f"Closed {sock}")
