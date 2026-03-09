# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

from .contact_info import ContactInfo
from .profile_data import ProfileData

# See: https://docs.linkupapi.com/api-reference/data-api/person-api/profile-info-direct#response
class UserInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    profile_url: str
    profile_data: ProfileData
    contact_info: ContactInfo | None = None
