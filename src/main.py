"""Main program for CLI tool"""

import click

from discovery import discover_lights


@click.group()
def main():
    pass


@main.command(name="list")
@click.option("-w", "--wait", "wait_time", type=int, default=1)
def list_lights(wait_time: int) -> None:
    discover_lights(wait_time=wait_time)


if __name__ == "__main__":
    main()
