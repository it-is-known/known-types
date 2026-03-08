# This is free and unencumbered software released into the public domain.

from datetime import datetime
from pydantic import BaseModel, ConfigDict

# See: https://api2.luma.com/user/profile?username=arto
class User(BaseModel):
    model_config = ConfigDict(extra="ignore")
    api_id: str  # e.g. "usr-kRR2ukq5fq50faV"
    avatar_url: str  # e.g. "https://images.lumacdn.com/avatars/4l/8b18b80a-a5c0-4682-a74e-a4ae5fe806a0",
    bio_short: str | None = None  # e.g. "Founder, ASIMOV Protocol. ex-NEAR Protocol"
    instagram_handle: str | None = None  # e.g. "arto.bendiken"
    is_verified: bool
    last_online_at: datetime  # e.g. "2026-02-27T22:30:28.589Z"
    linkedin_handle: str | None = None  # e.g. "/in/arto"
    name: str  # e.g. "Arto Bendiken"
    tiktok_handle: str | None = None
    timezone: str  # e.g. "America/Los_Angeles"
    twitter_handle: str | None = None  # e.g. "bendiken"
    username: str | None = None  # e.g. "arto"
    website: str | None = None  # e.g. "https://ar.to"
    youtube_handle: str | None = None
