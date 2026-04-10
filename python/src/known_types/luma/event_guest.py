# This is free and unencumbered software released into the public domain.


from typing import Any

from pydantic import BaseModel, ConfigDict

from .user import User


# See: https://api2.luma.com/event/get-guest-list?event_api_id=evt-cICd4IfvcuvlnjA&ticket_key=<KEY>
class EventGuest(BaseModel):
    model_config = ConfigDict(extra="ignore")
    api_id: str
    user: User
    num_tickets_registered: int
    section_label: Any
