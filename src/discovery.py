"""Automatic discovery of available lights on the network"""

import socket
import time

from pydantic import ValidationError

from messages import RegistrationRequest, RegistrationResponse

PORT = 38899
BROADCAST_IP = "255.255.255.255"


DISCOVERY_BROADCAST_MSG = RegistrationRequest.new_with_dummy_phone().to_message_bytes()


def discover_lights(wait_time: int) -> None:
    sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
    sock.setsockopt(socket.SOL_SOCKET, socket.SO_BROADCAST, 1)
    sock.bind(("", PORT))
    sock.setblocking(False)

    try:
        sock.sendto(DISCOVERY_BROADCAST_MSG, (BROADCAST_IP, PORT))
        time.sleep(wait_time)

        while True:
            try:
                data, address = sock.recvfrom(128)

                try:
                    RegistrationRequest.model_validate_json(data)
                    continue
                except ValidationError:
                    pass

                response = RegistrationResponse.model_validate_json(data)
                print(response)
            except BlockingIOError:
                break

    finally:
        sock.close()
