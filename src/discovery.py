"""Automatic discovery of available lights on the network"""

import logging
import socket
import time

from messages import RegistrationRequest, RegistrationResponse

__logger__ = logging.getLogger(__name__)

PORT = 38899
BROADCAST_IP = "255.255.255.255"

MAX_EXPECTED_MESSAGE_SIZE = 4096


DISCOVERY_BROADCAST_MSG = RegistrationRequest.new_with_dummy_phone().to_message_bytes()


def _initialize_socket_for_broadcast(port: int):
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    sock.bind(("", port))
    sock.setblocking(False)
    return sock


def discover_lights(wait_time: int) -> None:
    try:
        sock = _initialize_socket_for_broadcast(PORT)
        __logger__.debug(f"initialized {sock}")

        sock.sendto(DISCOVERY_BROADCAST_MSG, (BROADCAST_IP, PORT))
        time.sleep(wait_time)

        while True:
            try:
                data, (ip_addr, port) = sock.recvfrom(MAX_EXPECTED_MESSAGE_SIZE)
                __logger__.debug(f"Received message from {ip_addr}:{port} - {data}")

                # Ignore the self-broadcasted message
                if data == DISCOVERY_BROADCAST_MSG:
                    continue

                response = RegistrationResponse.model_validate_json(data)
                print(f"{ip_addr:10s} - {response.result.mac}")

            except BlockingIOError:
                break
    finally:
        sock.close()
        __logger__.debug(f"Closed {sock}")
