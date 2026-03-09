# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class ContactInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    email: str | None = None
    phone: str | None = None
    website: str | None = None
