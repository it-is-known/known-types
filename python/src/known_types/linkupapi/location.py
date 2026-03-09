# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

from .address import Address

class Location(BaseModel):
    address: Address
    streetAddressOptOut: bool = False
    description: str
    headquarter: bool = False
