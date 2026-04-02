# This is free and unencumbered software released into the public domain.


from datetime import datetime
from typing import Any

from pydantic import BaseModel, ConfigDict


# See: https://api2.luma.com/event/get?event_api_id=9n47y8c4
class EventInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    api_id: str  # "evt-vLOGwkM6yUlpS2u",
    calendar_api_id: str  # "cal-0Kku4XhepmL8zgF",
    cover_url: str  # "https://images.lumacdn.com/event-covers/33/4c690c9f-8d34-4f59-bc95-7b7fe0f0d6c8.png",
    end_at: datetime  # "2026-04-10T02:30:00.000Z",
    event_type: str  # "independent",
    hide_rsvp: bool  # false,
    location_type: str  # "offline",
    name: str  # "ASIMOV DevLabs #7: Context Graphs & Personal Intelligence",
    one_to_one: bool  # true,
    recurrence_id: str | None = None  # null,
    show_guest_list: bool  # true,
    start_at: datetime  # "2026-04-10T00:00:00.000Z",
    timezone: str  # "America/Los_Angeles",
    url: str  # "y1eszitt",
    user_api_id: str  # "usr-mxMIRXtTzPBCpiv",
    visibility: str  # "public",
    virtual_info: dict[str, Any]  # { "has_access": false },
    geo_address_info: dict[str, Any] | None = (
        None  # { "city": "San Francisco", "type": "google", "region": "California", "address": "Cole Frieman & Mallon LLP", "country": "United States", "place_id": "ChIJWTGPjmaAhYARrNgYc8IL0eQ", "localized": { "en-GB": { "city": "San Francisco", "region": "California", "address": "Cole Frieman & Mallon LLP", "country": "United States", "city_state": "San Francisco, California", "sublocality": "Financial District", "full_address": "Cole Frieman & Mallon LLP, 201 California St # 350, San Francisco, CA 94111, USA", "short_address": "201 California St # 350, San Francisco" } }, "city_state": "San Francisco, California", "description": "", "sublocality": "Financial District", "country_code": "US", "full_address": "Cole Frieman & Mallon LLP, 201 California St # 350, San Francisco, CA 94111, USA", "short_address": "201 California St # 350, San Francisco", "apple_maps_place_id": null, "mode": "shown" },
    )
    geo_address_visibility: str  # "public",
    coordinate: dict[str, float] | None = (
        None  # { "longitude": -122.3991947, "latitude": 37.793199699999995 },
    )
    waitlist_enabled: bool  # false,
    waitlist_status: str  # "disabled"
