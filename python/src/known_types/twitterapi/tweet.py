# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

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
    createdAt: str
    lang: str
    bookmarkCount: int
    isReply: bool
    inReplyToId: int | None = None
    conversationId: int
