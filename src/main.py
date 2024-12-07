"""Main program for CLI tool"""

import logging

import click

from discovery import discover_lights


@click.group()
@click.option("--verbose/", is_flag=True)
def main(verbose: bool):
    logging.basicConfig(
        level=logging.DEBUG if verbose else logging.INFO, handlers=[logging.StreamHandler()]
    )


@main.command(name="list")
@click.option("-w", "--wait", "wait_time", type=int, default=1)
def list_lights(wait_time: int) -> None:
    discover_lights(wait_time=wait_time)


if __name__ == "__main__":
    main()
