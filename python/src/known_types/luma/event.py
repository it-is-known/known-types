# This is free and unencumbered software released into the public domain.

from datetime import datetime
from typing import Any

from pydantic import BaseModel, ConfigDict, Field

from .calendar_info import CalendarInfo
from .event_info import EventInfo
from .user import User


class Event(BaseModel):
    model_config = ConfigDict(extra="ignore")
    api_id: str  # "evt-ZiAqg9LRqhPjDho",
    accepts_usdc_for_usd: bool  # false,
    calendar: CalendarInfo
    meta_pixel_id: str | None = None  # null,
    coupon: str | None = None  # null,
    google_measurement_id: str | None = None  # null,
    stripe_account_id: str | None = None  # null,
    payment_methods: list[Any] = Field(default_factory=list)  # [],
    event: EventInfo
    start_at: datetime  # "2026-04-01T22:00:00.000Z",
    guest_data: dict[
        str, Any
    ]  # { "email": null, "name": null, "ticket_key": null, "approval_status": null, "proxy_key": null, "event_tickets": [], "payments": [] },
    featured_guests: list[User] = Field(default_factory=list)
    has_available_ticket_types: bool  # true,
    refund_policy: str | None = None  # null,
    guest_count: int  # 95,
    ticket_count: int  # 95,
    hosts: list[User] = Field(default_factory=list)
    referred_by: str | None = None  # null,
    cover_image: dict[
        str, Any
    ]  # { "colors": ["#faf4f0", "#8dbfd8", "#d6482b"], "palette": { "neutral": [ { "color": "#faf4f0", "percentage": 48.29 } ], "vibrant": [ { "color": "#d6482b", "percentage": 3.18 }, { "color": "#8dbfd8", "percentage": 3.33 } ] } },
    sessions: list[Any]  # [],
    ticket_types: list[dict[str, Any]] = Field(
        default_factory=list
    )  # [ { "api_id": "evtticktyp-o4ZnHU7EckO74HD", "cents": null, "currency": null, "description": null, "ethereum_token_requirements": [], "event_api_id": "evt-ZiAqg9LRqhPjDho", "is_flexible": false, "is_hidden": false, "max_capacity": null, "membership_restriction": null, "min_cents": null, "position": "8", "name": "Standard", "require_approval": false, "type": "free", "valid_end_at": null, "valid_start_at": null, "num_tickets_registered": 95, "currency_info": null, "num_guests": 95, "spots_remaining": null, "is_disabled": false } ],
    featured_infos: list[dict[str, Any]] = Field(
        default_factory=list
    )  # [ { "type": "calendar", "avatar_url": "https://images.lumacdn.com/calendars/9k/5501eea1-5914-4a19-9658-305602cf7147.png", "name": "OpenClaw Meetups", "path": "/claw", "calendar_api_id": "cal-iOipAs7mv59Hbuz", "calendar_type": "calendar_global" } ],
    categories: list[dict[str, Any]] = Field(
        default_factory=list
    )  # [ { "api_id": "cat-ai", "description": "Join a hackathon, learn about LLMs and prompt engineering, or connect with other AI practitioners.", "event_count": 3381, "hero_image_desktop_url": "https://images.lumacdn.com/discovery/ai-square.png", "icon_url": "https://images.lumacdn.com/discovery/ai-icon.png", "name": "AI", "page_title": "AI Events", "simple_icon_url": "https://images.lumacdn.com/discovery/ai-icon-simple.png", "slug": "ai", "social_image_url": "https://images.lumacdn.com/discovery/ai-social.png", "subscriber_count": 59774, "tint_color": "#dd7aa4" } ],
    ticket_info: dict[
        str, Any
    ]  # { "price": null, "is_free": true, "max_price": null, "is_sold_out": false, "spots_remaining": 5, "is_near_capacity": true, "require_approval": false, "currency_info": null },
    subscribed_to_calendar: bool  # false,
    event_invite: str | None = None  # null,
    manager_info: str | None = None  # null,
    guest_info: str | None = None  # null,
    host_info: str | None = None  # null,
    sold_out: bool  # false,
    locale: str  # "en",
    theme_meta: dict[str, Any]  # { "theme": "legacy" },
    tint_color: str  # "#f8f4f0",
    can_register_for_multiple_tickets: bool  # false,
    font_title: str | None = None  # "new-spirit",
    description_mirror: dict[
        str, Any
    ]  # { "type": "doc", "content": [ { "type": "paragraph", "content": [ { "text": "OpenClaw & Governance", "type": "text", "marks": [ { "type": "bold" } ] }, { "text": " is a fireside chat exploring how AI agent systems are actually being built and where they may be heading.", "type": "text" } ] }, { "type": "paragraph", "content": [ { "text": "We’ll look at OpenClaw as an open-source operating system for AI agents, with a focus on practical use, real-world applications, and the governance decisions shaping its future.", "type": "text" } ] }, { "type": "paragraph", "content": [ { "text": "This session is for builders, developers, and curious minds who want a clearer understanding of intelligent automation and the open systems driving it forward.", "type": "text" } ] }, { "type": "paragraph", "content": [ { "text": "Versión en español", "type": "text", "marks": [ { "type": "bold" } ] }, { "type": "hard_break" }, { "text": "OpenClaw & Governance", "type": "text", "marks": [ { "type": "bold" } ] }, { "text": " es una conversación íntima sobre cómo se están construyendo los sistemas de agentes de IA y hacia dónde podrían dirigirse.", "type": "text" } ] }, { "type": "paragraph", "content": [ { "text": "Exploraremos OpenClaw como un sistema operativo open-source para agentes de IA, con enfoque en usos prácticos, aplicaciones reales y las decisiones de gobernanza que están dando forma a su futuro.", "type": "text" } ] }, { "type": "paragraph", "content": [ { "text": "Esta sesión es para builders, developers y mentes curiosas que quieran entender mejor la automatización inteligente y los sistemas abiertos que la impulsan.", "type": "text" } ] }, { "type": "paragraph", "content": [ { "text": "Hosted by Future Tech Collective PR as a satellite event of Tokenize LATAM San Juan.", "type": "text", "marks": [ { "type": "bold" } ] } ] } ] },
    eth_address_requirement: str | None = None  # null,
    name_requirement: str  # "full-name",
    phone_number_requirement: str | None = None  # null,
    solana_address_requirement: str | None = None  # null,
    registration_questions: list[Any] = Field(default_factory=list)  # [],
    is_flagged: bool  # false,
    is_primary_calendar_admin: bool | None = None  # false,
    show_unlock_code_option: bool  # false,
    has_multiple_ticket_types: bool  # false,
    membership_tiers: list[Any] = Field(default_factory=list)  # [],
    membership_info: str | None = None  # null,
    waitlist_active: bool  # false,
    route: str | None = None  # null,
    featured_city: dict[str, Any] | None = None  # null,
    role: str | None = None  # null
