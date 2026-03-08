# This is free and unencumbered software released into the public domain.

from datetime import datetime
from pydantic import BaseModel, ConfigDict

from .user import User

# See: https://api2.luma.com/user/profile?username=arto
class Profile(BaseModel):
    model_config = ConfigDict(extra="ignore")
    event_attended_count: int
    event_hosted_count: int
    event_together_count: int
    joined_at: datetime  # e.g. "2024-04-17T14:12:57.074Z"
    user: User
    block_robots: bool
