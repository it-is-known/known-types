# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class DurationSetting(BaseModel):
    model_config = ConfigDict(extra="ignore")
    Duration: int  # 30
    Price: int  # 4000
    Discount: int  # 25
    Active: bool  # false
