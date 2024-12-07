"""Main program for CLI tool"""

import logging

import click

from messages import RegistrationRequest, RegistrationResponse
from network import broadcast_udp

DISCOVERY_BROADCAST_MESSAGE = RegistrationRequest.new_with_dummy_phone().to_message_bytes()


@click.group()
@click.option("--verbose/", is_flag=True)
def main(verbose: bool):
    logging.basicConfig(
        level=logging.DEBUG if verbose else logging.INFO, handlers=[logging.StreamHandler()]
    )


@main.command(name="list")
@click.option("-w", "--wait", "wait_time", type=int, default=1)
def list_lights(wait_time: int) -> None:
    print("IP Address | MAC Address\n------------------------")
    for data, (ip_addr, _) in broadcast_udp(DISCOVERY_BROADCAST_MESSAGE, wait_time):
        response = RegistrationResponse.model_validate_json(data)
        print(f"{ip_addr:10s} | {response.result.mac}")


if __name__ == "__main__":
    main()
