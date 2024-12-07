"""Data model for WiZ UDP messages"""

from abc import ABC
from typing import Annotated, Literal

from pydantic import BaseModel as BaseModel_
from pydantic import BeforeValidator, ConfigDict
from pydantic_extra_types.mac_address import MacAddress as MacAddress_


def parse_mac_address(mac: str) -> str:
    chars = list(mac)

    def _iterate_with_colon_insertion():
        while chars:
            yield chars.pop(0)
            yield chars.pop(0)
            if chars:
                yield ":"

    return "".join(_iterate_with_colon_insertion())


MacAddress = Annotated[MacAddress_, BeforeValidator(parse_mac_address)]


class BaseModel(BaseModel_, ABC):
    model_config = ConfigDict(extra="forbid")

    def to_message_bytes(self) -> bytes:
        return self.model_dump_json(by_alias=True).encode()


class GetPilotBase(BaseModel, ABC):
    method: Literal["getPilot"] = "getPilot"


class GetPilotRequest(GetPilotBase): ...


class GetPilotResponse(GetPilotBase):
    class _Result(BaseModel):
        mac: MacAddress
        rssi: int
        state: bool
        sceneId: int
        r: int | None = None
        g: int | None = None
        b: int | None = None
        c: int | None = None
        w: int | None = None
        dimming: int

    env: Literal["pro"]
    result: _Result
