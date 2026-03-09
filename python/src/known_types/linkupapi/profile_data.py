# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

from .education import Education
from .experience import Experience

class ProfileData(BaseModel):
    model_config = ConfigDict(extra="ignore")
    public_id: str
    first_name: str
    last_name: str
    headline: str
    location: str
    summary: str
    industry: str
    experience: list[Experience]
    education: list[Education]
    skills: list[str]
    profile_picture_url: str | None = None
