# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

class EmployeeCountRange(BaseModel):
    start: int
    end: int
