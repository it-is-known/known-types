# This is free and unencumbered software released into the public domain.

from datetime import datetime
from pydantic import BaseModel, ConfigDict

# See: https://api2.luma.com/event/get?event_api_id=9n47y8c4
class Event(BaseModel):
    model_config = ConfigDict(extra="ignore")
    api_id: str  # e.g. "evt-LLaekW1XcxeHwjU"
    start_at: datetime
    end_at: datetime
    name: str
    show_guest_list: bool
    url: str
    # sold_out: bool
    # guest_count: int
    # ticket_count: int
