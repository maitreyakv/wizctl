"""Main program for CLI tool"""

import logging

import click
import polars as pl

from messages import GetPilotRequest, GetPilotResponse
from network import broadcast_udp


@click.group()
@click.option("--verbose/", is_flag=True)
def main(verbose: bool):
    logging.basicConfig(
        level=logging.DEBUG if verbose else logging.INFO, handlers=[logging.StreamHandler()]
    )


@main.command(name="list")
@click.option("-w", "--wait", "wait_time", type=int, default=1)
def list_lights(wait_time: int) -> None:
    request_data = GetPilotRequest().to_message_bytes()
    pilots = (
        pl.DataFrame(
            [
                {"ip": ip_addr} | GetPilotResponse.model_validate_json(data).result.model_dump()
                for data, (ip_addr, _) in broadcast_udp(request_data, wait_time)
            ]
        )
        .with_columns(
            rgbcw=pl.format(
                "{}/{}/{} - {}/{}", pl.col("r"), pl.col("g"), pl.col("b"), pl.col("c"), pl.col("w")
            )
        )
        .drop("r", "g", "b", "c", "w")
    )
    print(pilots)


if __name__ == "__main__":
    pl.Config.set_tbl_hide_column_data_types(True)
    pl.Config.set_tbl_hide_dataframe_shape(True)
    main()
