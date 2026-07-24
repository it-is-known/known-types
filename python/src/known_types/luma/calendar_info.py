# This is free and unencumbered software released into the public domain.

from datetime import datetime
from typing import Any

from pydantic import BaseModel, ConfigDict, Field

from .event_info import EventInfo
from .user import User


# See: https://api2.luma.com/url?url=claw
class CalendarInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    access_level: str  # "public",
    api_id: str  # "cal-iOipAs7mv59Hbuz",
    avatar_url: str  # "https://images.lumacdn.com/calendars/9k/5501eea1-5914-4a19-9658-305602cf7147.png",
    coordinate: dict[str, float] | None = None  # null,
    cover_image_url: str | None = (
        None  # "https://images.lumacdn.com/calendar-cover-images/tr/e3e88858-4da4-475e-a60b-ef385fffdfda.png",
    )
    description_short: str | None = (
        None  # "Discover community meetups for OpenClaw around the world.",
    )

    event_submission_restriction: str  # "open",
    geo_city: str | None = None  # null,
    geo_country: str | None = None  # null,
    geo_region: str | None = None  # null,
    google_measurement_id: str | None = None  # null,
    instagram_handle: str | None = None  # null,
    is_blocked: bool  # false,
    launch_status: str  # "launched",
    linkedin_handle: str | None = None  # null,
    luma_plan: str  # "free",
    luma_plus_active: bool  # false,
    meta_pixel_id: str | None = None  # null,
    name: str  # "OpenClaw Meetups",
    personal_user_api_id: str | None = None  # null,
    refund_policy: dict[str, Any] | None  # null,
    slug: str | None = None  # "claw",
    social_image_url: str | None = (
        None  # "https://images.lumacdn.com/calendar-cover-images/bb/1d12a4f3-8e63-4420-b2fb-f9a1f53b5267.png",
    )
    stripe_account_id: str | None = None  # null,
    tax_config: str | None = None  # null,
    tiktok_handle: str | None = None  # null,
    timezone: str | None = None  # "America/New_York",
    tint_color: str  # "#6bb2c1",
    track_meta_ads_from_luma: bool  # false,
    twitter_handle: str | None = None  # "openclaw",
    verified_at: datetime | None = None  # "2026-02-09T18:10:45.762Z",
    website: str | None = None  # "https://openclaw.ai/",
    youtube_handle: str | None = None  # null,
    is_personal: bool  # false,
    personal_user: dict[str, Any] | None = None  # null


# See: https://api2.luma.com/url?url=claw
class CalendarEvent(BaseModel):
    model_config = ConfigDict(extra="ignore")
    api_id: str  # "calev-E6FAuBGjZH2AlVG",
    event: EventInfo
    cover_image: Any  # { "colors": [ "#f5f0e8", "#c42b2a", "#744bf7", "#c4f91c" ], "palette": { "neutral": [ { "color": "#f5f0e8", "percentage": 64.42 } ], "vibrant": [ { "color": "#c42b2a", "percentage": 3.7 }, { "color": "#744bf7", "percentage": 0.18 }, { "color": "#c4f91c", "percentage": 0.04 } ] } },
    calendar: CalendarInfo
    start_at: datetime  # "2026-04-01T11:30:00.000Z",
    hosts: list[User] = Field(default_factory=list)
    guest_count: int  # 192,
    ticket_count: int  # 192,
    ticket_info: dict[
        str, Any
    ]  # { "price": null, "is_free": true, "max_price": null, "is_sold_out": true, "spots_remaining": 0, "is_near_capacity": false, "require_approval": false, "currency_info": null },
    featured_guests: list[User] = Field(default_factory=list)
    manager_info: str | None = None  # null,
    guest_info: str | None = None  # null,
    host_info: str | None = None  # null,
    waitlist_active: bool  # true,
    featured_city: dict[str, Any] | None = None  # null,
    role: str | None = None  # null,
    calendar_api_id: str  # "cal-iOipAs7mv59Hbuz",
    is_manager: bool  # false,
    platform: str  # "luma",
    status: str  # "approved",
    submitted_by_user_api_id: str  # "usr-9Xe6WZFxIvTzAMK",
    tags: list[Any] = Field(default_factory=list)  # []
