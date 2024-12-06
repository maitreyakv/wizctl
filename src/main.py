"""Main program for CLI tool"""

import click


@click.command()
def hello():
    print("hello!")


if __name__ == "__main__":
    hello()
