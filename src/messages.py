"""Data model for WiZ UDP messages"""

from abc import ABC
from typing import Literal

from pydantic import BaseModel as BaseModel_
from pydantic import ConfigDict


class BaseModel(BaseModel_, ABC):
    model_config = ConfigDict(extra="forbid")

    def to_message_bytes(self) -> bytes:
        return self.model_dump_json(by_alias=True).encode()


class GetPilotBase(BaseModel, ABC):
    method: Literal["getPilot"] = "getPilot"


class GetPilotRequest(GetPilotBase): ...


class GetPilotResponse(GetPilotBase):
    class _Result(BaseModel):
        mac: str
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
