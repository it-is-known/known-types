# This is free and unencumbered software released into the public domain.

from datetime import datetime
from typing import Any

from pydantic import BaseModel, ConfigDict


# See: https://docs.twitterapi.io/api-reference/endpoint/get_list_members#response-members
class ListMember(BaseModel):
    model_config = ConfigDict(extra="ignore")
    id: int
    name: str
    userName: str
    location: str | None = None
    url: str | None = None
    description: str | None = None
    protected: bool
    isVerified: bool
    isBlueVerified: bool
    verifiedType: Any
    followers: int
    following: int
    favouritesCount: int
    statusesCount: int
    mediaCount: int
    createdAt: datetime
    coverPicture: str | None = None
    profilePicture: str
    canDm: bool
    isAutomated: bool
    automatedBy: str | None = None
