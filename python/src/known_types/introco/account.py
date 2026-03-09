# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class Account(BaseModel):
    model_config = ConfigDict(extra="ignore")
    Id: int  # 177804
    FirstName: str  # "Alexis"
    LastName: str  # "Ohanian"
    Username: str  # ""
    Title: str  # "Tech Entrepreneur + Investor"
    Description: str  # "👨‍💻 Co-founder of Reddit. First batch of Y Combinator (Summer 2005) and led the company to a sale to Condé Nast in 2006, returned as Exec Chair in 2014 to help lead the turnaround, then left in 2018 to do venture capital fulltime.\n\n🦄 I’m an investor in startups —almost always at the earliest possible stage— first as an angel investor, then co-founder of Initialized, before splitting the firm in half to found Seven Seven Six.\n\nFirst round investments include: Coinbase (NYSE: COIN), Opendoor (NASDAQ: OPEN), Instacart (NYSE: ART), Reddit (NYSE: RDDT), Gusto, Flexport, Ro, Flock Safety, Athelas, Rippling, Patreon, GOAT, Cruise, Feastables, and many more.\n\n🗺️ Current: Founder of 7️⃣7️⃣6️⃣, a technology company that deploys venture capital.\n\n💪 Strengths:\n- Startups \n- Investing\n- Company Culture\n- Early Stage Marketing\n- Growth Tactics\n- Operations\n- Fundraising\n- Hiring & Managing\n\nLooking forward to helping you in your business journey! I love helping entrepreneurs and my proceeds will go to my foundation that you can learn more about at 776.org"
    ShortDescription: str  # "Founder of Reddit, Initialized, & 7️⃣7️⃣6️⃣ \n(100% to charity)"
    Avatar: str  # "https://media.intro.co/avatars/177804F4nhK4N2.jpg"
    Instagram: str  # "AlexisOhanian"
    Twitter: str  # "AlexisOhanian"
    LinkedIn: str  # ""
    YouTube: str  # ""
    TikTok: str  # "AlexisOhanian"
    Url: str  # "https://intro.co/AlexisOhanian"
