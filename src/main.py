"""Main program for CLI tool"""

import click


@click.group()
def main():
    pass


@main.command(name="list")
def list_lights() -> None:
    print("listing...")


if __name__ == "__main__":
    main()
