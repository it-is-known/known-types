# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_by_username#response-data
class UserInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    id: int
    name: str
    userName: str
    location: str
    url: str
    description: str
    entities: dict
    protected: bool
    isVerified: bool
    isBlueVerified: bool
    verifiedType: str | None = None
    followers: int
    following: int
    favouritesCount: int
    statusesCount: int
    mediaCount: int
    createdAt: str
    coverPicture: str | None = None
    profilePicture: str
    canDm: bool
    affiliatesHighlightedLabel: dict
    isAutomated: bool
    automatedBy: str | None = None
    pinnedTweetIds: list[str]
