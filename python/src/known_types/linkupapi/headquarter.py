# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

from .address import Address

class Headquarter(Address):
    pass
