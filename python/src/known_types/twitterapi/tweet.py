# This is free and unencumbered software released into the public domain.

from datetime import datetime
from typing import Annotated, Any

from pydantic import BaseModel, BeforeValidator, ConfigDict, field_validator


def _parse_timestamp(value: Any) -> Any:
    # e.g. `Tue Dec 10 07:00:30 +0000 2024`, the format used everywhere except
    # the user info endpoint. Empty on the stub tweets X leaves at the end of a
    # quote chain.
    if not isinstance(value, str):
        return value
    if not value:
        return None
    try:
        return datetime.strptime(value, "%a %b %d %H:%M:%S %z %Y")
    except ValueError:
        return value

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_timeline#response-tweets
class TweetAuthor(BaseModel):
    model_config = ConfigDict(extra="ignore")
    id: int
    userName: str
    name: str
    createdAt: Annotated[datetime, BeforeValidator(_parse_timestamp)]
    followers: int
    following: int
    statusesCount: int
    profilePicture: str | None = None

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_timeline#response-tweets
class TweetVideoVariant(BaseModel):
    model_config = ConfigDict(extra="ignore")
    url: str
    content_type: str
    bitrate: int | None = None # absent on the HLS playlist variant

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_timeline#response-tweets
class TweetVideoInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    variants: list[TweetVideoVariant] = []

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_timeline#response-tweets
class TweetMedia(BaseModel):
    model_config = ConfigDict(extra="ignore")
    type: str
    media_url_https: str | None = None
    video_info: TweetVideoInfo | None = None

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_timeline#response-tweets
class TweetExtendedEntities(BaseModel):
    model_config = ConfigDict(extra="ignore")
    media: list[TweetMedia] = []

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_last_tweets#response-tweets
class Tweet(BaseModel):
    model_config = ConfigDict(extra="ignore")
    # type: str
    id: int
    url: str
    # twitterUrl: str
    text: str
    source: str
    retweetCount: int
    replyCount: int
    likeCount: int
    quoteCount: int
    viewCount: int
    createdAt: Annotated[datetime | None, BeforeValidator(_parse_timestamp)] = None
    lang: str
    bookmarkCount: int
    isReply: bool
    inReplyToId: int | None = None
    conversationId: int | None = None
    author: TweetAuthor | None = None
    extendedEntities: TweetExtendedEntities | None = None
    quoted_tweet: "Tweet | None" = None
    retweeted_tweet: "Tweet | None" = None

    # X reports these as an empty string or an empty object rather than null on
    # the stub tweets it leaves at the end of a quote chain.
    @field_validator("conversationId", "inReplyToId", "author", mode="before")
    @classmethod
    def _empty_is_null(cls, value: Any) -> Any:
        return value or None
