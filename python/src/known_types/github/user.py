# This is free and unencumbered software released into the public domain.

from datetime import datetime
from pydantic import BaseModel, ConfigDict

# See: https://docs.github.com/en/rest/users/users?apiVersion=2022-11-28#get-a-user
class User(BaseModel):
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
    name: str | None = None  # "monalisa octocat"
    company: str | None = None  # "GitHub"
    blog: str | None = None  # "https://github.com/blog"
    location: str | None = None  # "San Francisco"
    email: str | None = None  # "octocat@github.com"
    notification_email: str | None = None  # "octocat@github.com"
    hireable: bool | None = None  # false
    bio: str | None = None  # "There once was..."
    twitter_username: str | None = None  # "monatheoctocat"
    public_repos: int  # 2
    public_gists: int  # 1
    followers: int  # 20
    following: int  # 0
    private_gists: int | None = None  # 1
    total_private_repos: int | None = None  # 2
    owned_private_repos: int | None = None  # 2
    collaborators: int | None = None  # 2
    created_at: datetime  # "2008-01-14T04:33:35Z"
    updated_at: datetime  # "2008-01-14T04:33:35Z"
