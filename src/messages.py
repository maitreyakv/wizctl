"""Data model for WiZ UDP messages"""

from abc import ABC
from typing import Literal, Self

from pydantic import BaseModel, Field


class BaseMessage(BaseModel, ABC):
    def to_message_bytes(self) -> bytes:
        return self.model_dump_json(by_alias=True).encode()


class RegistrationBase(BaseMessage, ABC):
    method: Literal["registration"] = "registration"


class RegistrationRequest(RegistrationBase):
    class _Params(BaseModel):
        register_: bool = Field(alias="register")
        phoneMac: str
        phoneIp: str

    params: _Params

    @classmethod
    def new_with_dummy_phone(cls) -> Self:
        return cls(params=cls._Params(register=False, phoneMac="000000000000", phoneIp="0.0.0.0"))


class RegistrationResponse(RegistrationBase):
    class _Result(BaseModel):
        mac: str
        success: bool

    env: Literal["pro"]
    result: _Result
