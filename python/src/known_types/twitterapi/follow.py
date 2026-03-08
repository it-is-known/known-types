# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_followers#response-followers
class Follower(BaseModel):
    model_config = ConfigDict(extra="ignore")
    id: int
    name: str
    screen_name: str
    userName: str
    location: str
    url: str | None = None
    description: str
    email: str | None = None
    protected: bool
    verified: bool
    followers_count: int
    following_count: int
    friends_count: int
    favourites_count: int
    statuses_count: int
    media_tweets_count: int
    created_at: str
    profile_banner_url: str | None = None
    profile_image_url_https: str
    can_dm: bool

# See: https://docs.twitterapi.io/api-reference/endpoint/get_user_followings#response-followings
Following = Follower
