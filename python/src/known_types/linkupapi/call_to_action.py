# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class CallToAction(BaseModel):
    displayText: str
    visible: bool
    type: str
    url: str
