from datetime import datetime
from pydantic import BaseModel, Field


class 0(BaseModel):
    0: str
    1: int
    2: list[str] = Field(default_factory=list)


class 1(BaseModel):
    0: str
    1: str
    2: str
    3: int
    4: bool
    5: str = ""


class 2(BaseModel):
    0: int
    1: str
    2: datetime
    3: str = "unknown"
    4: list[0] = Field(default_factory=list)
    5: list[1] | None = None
    6: list[str] = Field(default_factory=list)


class 3(BaseModel):
    0: int
    1: int
    2: list[2]