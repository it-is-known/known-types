# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class Experience(BaseModel):
    model_config = ConfigDict(extra="ignore")
    company: str
    title: str
    description: str
    start_date: str
    end_date: str
