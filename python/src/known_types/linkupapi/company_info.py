# This is free and unencumbered software released into the public domain.

from pydantic import BaseModel, ConfigDict

from .call_to_action import CallToAction
from .employee_count_range import EmployeeCountRange
from .headquarter import Headquarter
from .location import Location

# See: https://docs.linkupapi.com/api-reference/data-api/company-api/company-info-direct#response
class CompanyInfo(BaseModel):
    model_config = ConfigDict(extra="ignore")
    name: str
    universalName: str
    description: str
    tagline: str
    websiteUrl: str
    industry: str
    employeeCount: int
    employeeCountRange: EmployeeCountRange
    headquarter: Headquarter
    logoUrl: str
    coverImageUrl: str
    callToAction: CallToAction | None = None
    locations: list[Location]
    organizationType: str
    defaultLocale: str
    followersCount: int
    pageMailbox: bool
    verified: bool
