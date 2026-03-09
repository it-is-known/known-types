# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

from .account import Account
from .duration_setting import DurationSetting

class Profile(Account):
    AllowRequests: bool  # false
    SessionPrice: int  # 2000
    SessionDuration: int  # 15
    DurationSettings: list[DurationSetting] | None = None
    # "SessionDescriptions": [
    #   "Ask three or more questions",
    #   "Tips on how to start a successful company",
    #   "Advice on getting your first 10,000 customers",
    #   "Growth hacks & jumpstarting growth"
    # ]
    # "ExampleQuestions": [
    #   "I’m thinking of starting a company. What are the next few things I should focus on?",
    #   "How do I know if my business idea will work?",
    #   "How would you approach growth for my business?",
    #   "What key metrics should I aim for at different stages to be a best in class company?"
    # ]
    Discount: int  # 0
    CharityName: str | None = None  # "776 Foundation"
    CharityPercent: int | None = None  # 100
    Rating: float  # 4.979591836734694
    RatingCount: int  # 49
    Verified: bool  # true
    TopExpert: bool  # true
    # "Favorited": false
    # "SubscriptionTiers": []
    # "ChatBoostPrice": 0
