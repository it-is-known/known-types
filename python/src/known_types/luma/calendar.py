# This is free and unencumbered software released into the public domain.

from datetime import datetime
from typing import Any

from pydantic import BaseModel, ConfigDict, Field

from .calendar_info import CalendarEvent, CalendarInfo


# See: https://api2.luma.com/url?url=claw
class Calendar(BaseModel):
    model_config = ConfigDict(extra="ignore")
    calendar: CalendarInfo
    featured_items: list[CalendarEvent] = Field(default_factory=list)
    has_upcoming_events: bool  # true,
    has_access: bool  # true,
    is_admin: bool  # false,
    calendar_member: str | None = None  # null,
    is_subscriber: bool  # false,
    tags: list[Any] = Field(default_factory=list)  # [],
    event_start_ats: list[datetime] = Field(
        default_factory=list
    )  # [ "2026-02-10T07:30:00.000Z" ],
    membership_tiers: list[Any] = Field(default_factory=list)  # [],
    categories: list[dict[str, Any]] = Field(
        default_factory=list
    )  # [ { "api_id": "cat-ai", "description": "Join a hackathon, learn about LLMs and prompt engineering, or connect with other AI practitioners.", "event_count": 3317, "hero_image_desktop_url": "https://images.lumacdn.com/discovery/ai-square.png", "icon_url": "https://images.lumacdn.com/discovery/ai-icon.png", "name": "AI", "page_title": "AI Events", "simple_icon_url": "https://images.lumacdn.com/discovery/ai-icon-simple.png", "slug": "ai", "social_image_url": "https://images.lumacdn.com/discovery/ai-social.png", "subscriber_count": 59611, "tint_color": "#dd7aa4" } ],
    membership_info: Any = None  # null,
    avatar_palette: dict[str, Any] | None = (
        None  # { "neutral": [ { "color": "#99cdd9", "percentage": 70.05 } ], "vibrant": [ { "color": "#e94f01", "percentage": 5.65 }, { "color": "#ed7d12", "percentage": 4.87 } ] },
    )
    can_subscribe: bool  # true
