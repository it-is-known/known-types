# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict


# See: https://docs.github.com/en/rest/users/followers?apiVersion=2022-11-28#list-followers-of-a-user
class Follower(BaseModel):
    model_config = ConfigDict(extra="ignore")
    login: str  # "octocat"
    id: int  # 1
    node_id: str  # "MDQ6VXNlcjE="
    avatar_url: str  # "https://github.com/images/error/octocat_happy.gif"
    gravatar_id: str | None = None  # ""
    url: str  # "https://api.github.com/users/octocat"
    html_url: str  # "https://github.com/octocat"
    followers_url: str  # "https://api.github.com/users/octocat/followers"
    following_url: str  # "https://api.github.com/users/octocat/following{/other_user}"
    gists_url: str  # "https://api.github.com/users/octocat/gists{/gist_id}"
    starred_url: str  # "https://api.github.com/users/octocat/starred{/owner}{/repo}"
    subscriptions_url: str  # "https://api.github.com/users/octocat/subscriptions"
    organizations_url: str  # "https://api.github.com/users/octocat/orgs"
    repos_url: str  # "https://api.github.com/users/octocat/repos"
    events_url: str  # "https://api.github.com/users/octocat/events{/privacy}"
    received_events_url: str  # "https://api.github.com/users/octocat/received_events"
    type: str  # "User"
    site_admin: bool  # false


# See: https://docs.github.com/en/rest/users/followers?apiVersion=2022-11-28#list-the-people-a-user-follows
Following = Follower
