from typing import List, Dict, Union
from enum import Enum

from .color import Color, ColorVec
from .tier import Tier


class Card:
    """
    represent the development cards
    """

    tier: Tier
    bonus: Color
    points: int
    requires: ColorVec

    def __init__(self, tier: int, bonus: str, points: int, requires: List[int]):
        self.tier = Tier(tier)
        self.bonus = Color.from_str(bonus)
        self.points = points
        self.requires = ColorVec(requires)


class CardPool:
    """
    represent the development cards pool
    """

    remaining: List[int]
    revealed: List[List[Card]]

    def __init__(self, remaining: List[int], revealed: List[List[Dict]]):
        self.remaining = remaining
        self.revealed = [[Card(**card) for card in tier] for tier in revealed]


class ReservedCardType(Enum):
    """
    Enum for the reserved card type.
    Reserved cards can be invisible if they are reserved from pool.
    """

    Visible = 0
    Invisible = 1

    def from_str(s: str) -> "ReservedCardType":
        """
        Convert a string to a ReservedCardType enum
        """
        s = s.lower()
        if s == "visible":
            return ReservedCardType.Visible
        elif s == "invisible":
            return ReservedCardType.Invisible
        else:
            raise ValueError(f"Invalid reserved card type: {s}")

    def __str__(self):
        return self.name

    def __repr__(self):
        return self.name


class CardView:
    """
    represent the reserved cards
    """

    type: ReservedCardType
    view: Union[Card, Tier]

    def __init__(self, type: str, view: Union[Dict, int]):
        self.type = ReservedCardType.from_str(type)
        if self.type == ReservedCardType.Visible:
            self.view = Card(**view)
        else:
            self.view = Tier(view)


class DevelopmentCards:
    """
    represent the development cards of a player
    """

    points: int
    bonus: ColorVec
    inner: List[List[Card]]

    def __init__(self, points: int, bonus: List[int], inner: List[List[Dict]]):
        self.points = points
        self.bonus = ColorVec(bonus)
        self.inner = [[Card(**card) for card in tier] for tier in inner]
