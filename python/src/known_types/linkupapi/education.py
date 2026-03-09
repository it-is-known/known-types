# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class Education(BaseModel):
    model_config = ConfigDict(extra="ignore")
    school: str
    degree: str
    field_of_study: str
    start_date: str
    end_date: str | None = None
