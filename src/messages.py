"""Data model for WiZ UDP messages"""

from abc import ABC
from typing import Literal, Self

from pydantic import BaseModel as BaseModel_
from pydantic import ConfigDict, Field, SerializationInfo, field_serializer
from pydantic_extra_types.mac_address import MacAddress


class BaseModel(BaseModel_, ABC):
    model_config = ConfigDict(extra="forbid")

    def to_message_bytes(self) -> bytes:
        return self.model_dump_json(by_alias=True).encode()


class RequestBase(BaseModel_, ABC): ...


class ResponseBase(BaseModel_, ABC): ...


class RegistrationBase(BaseModel, ABC):
    method: Literal["registration"] = "registration"


class RegistrationRequest(RegistrationBase, RequestBase):
    class _Params(BaseModel):
        register_: bool = Field(alias="register")
        phoneMac: MacAddress
        phoneIp: str

        @field_serializer("phoneMac")
        def serialize_phoneMac(self, phoneMac: MacAddress, _info: SerializationInfo):
            return str(phoneMac).replace(":", "")

    params: _Params

    @classmethod
    def new_with_dummy_phone(cls) -> Self:
        return cls(
            params=cls._Params(
                register=False,
                phoneMac="00:00:00:00:00:00",
                phoneIp="0.0.0.0",
            )
        )


class RegistrationResponse(RegistrationBase, ResponseBase):
    class _Result(BaseModel):
        mac: str
        success: bool

    env: Literal["pro"]
    result: _Result
