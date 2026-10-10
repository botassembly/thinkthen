"""Ordinary immutable input values; native Request owns their validation."""
from __future__ import annotations
from dataclasses import dataclass

class Absent:
    def __repr__(self): return 'ABSENT'
ABSENT = Absent()

@dataclass(frozen=True, repr=False, kw_only=True)
class QuestionSource:
    body: object | None = None
    path: str | None = None
    name: str | None = None
    reference: str | None = None
    none: bool = False
    raw: str | None = None

    def __repr__(self): return '<QuestionSource: content withheld>'

@dataclass(frozen=True, repr=False, kw_only=True)
class Image:
    media: str
    data: bytes
    def __repr__(self): return '<Image: bytes withheld>'

@dataclass(frozen=True, repr=False, kw_only=True)
class Item:
    value: object = None
    text: bool = False
    images: tuple[Image, ...] = ()
    context: object | Absent = ABSENT
    options: tuple[tuple[str, object | Absent], ...] | Absent = ABSENT
    image_only: bool = False
    def __repr__(self): return '<Item: content withheld>'

@dataclass(frozen=True, repr=False)
class Records:
    items: tuple[Item, ...]
    def __repr__(self): return '<Records: content withheld>'

@dataclass(frozen=True, repr=False, kw_only=True)
class Files:
    paths: tuple[str, ...]
    unit: str = 'line'
    window: int | Absent = ABSENT
    media: str = 'text'
    jsonl: bool = False
    def __repr__(self): return '<Files: content withheld>'

