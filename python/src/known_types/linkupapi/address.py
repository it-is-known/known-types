# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class Address(BaseModel):
    country: str | None = None
    geographicArea: str | None = None
    city: str | None = None
    line1: str | None = None
    line2: str | None = None
    postalCode: str | None = None
