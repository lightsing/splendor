from enum import Enum
from typing import Set, Dict, Optional, Union, List

from .color import Color, ColorVec
from .card import Tier

PlayerAction = Union[
    "TakeTokenAction",
    "ReserveCardAction",
    "BuyCardAction",
    "NopAction",
]


class NopAction:
    """
    represent the action of doing nothing
    """

    def to_json(self) -> Dict:
        return {"type": "nop"}


class TakeTokenAction:
    """
    represent the action of taking tokens
    """

    class Type(Enum):
        THREE_DIFFERENT = "three_different"
        TWO_SAME = "two_same"

    _type: Type
    tokens: ColorVec

    def __init__(self, _type: Type, tokens: ColorVec):
        self._type = _type
        self.tokens = tokens

    @classmethod
    def three_different(cls, tokens: Union[Set[Color], List[Color]]):
        """
        create a TakeTokenAction with at most three different tokens
        """
        assert len(tokens) <= 3
        vec = ColorVec.empty()
        if isinstance(tokens, list):
            tokens = set(tokens)
        for token in tokens:
            vec[token] = 1
        return cls(cls.Type.THREE_DIFFERENT, vec)

    @classmethod
    def two_same(cls, token: Color):
        """
        create a TakeTokenAction with two same tokens
        """
        tokens = ColorVec.empty()
        tokens[token] = 2
        return cls(cls.Type.TWO_SAME, tokens)

    def to_json(self) -> Dict:
        return {
            "type": "take_tokens",
            "action": {"type": self._type.value, "tokens": self.tokens.to_json()},
        }


class ReserveCardAction:
    """
    represent the action of reserving a card
    """

    def __init__(self, tier: Tier, idx: Optional[int]):
        self.tier = tier
        self.idx = idx

    @classmethod
    def from_revealed(cls, tier: Tier, idx: int):
        """
        create a ReserveCardAction from the revealed cards
        """
        assert 0 <= idx < 4

        return cls(tier, idx)

    @classmethod
    def from_pool(cls, tier: Tier):
        """
        create a ReserveCardAction from the card pool
        """
        return cls(tier, None)

    def to_json(self) -> Dict:
        return {
            "type": "reserve_card",
            "action": {
                "tier": self.tier.value,
                "idx": self.idx,
            },
        }


class BuyCardAction:
    """
    represent the action of buying a card
    """

    class SourceType(Enum):
        REVEALED = "revealed"
        RESERVED = "reserved"

    class RevealedCardLocation:
        tier: Tier
        idx: int

        def __init__(self, tier: Tier, idx: int):
            self.tier = tier
            self.idx = idx

        def to_json(self) -> Dict:
            return {
                "tier": self.tier.value,
                "idx": self.idx,
            }

    class ReservedCardLocation:
        idx: int

        def __init__(self, idx: int):
            self.idx = idx

        def to_json(self) -> int:
            return self.idx

    source_type: SourceType
    location: Union[RevealedCardLocation, ReservedCardLocation]
    uses: ColorVec

    def __init__(
        self,
        source_type: SourceType,
        location: Union[RevealedCardLocation, ReservedCardLocation],
        uses: ColorVec,
    ):
        self.source_type = source_type
        self.location = location
        self.uses = uses

    @classmethod
    def from_revealed(cls, tier: Tier, idx: int, uses: ColorVec):
        """
        create a BuyCardAction from the revealed cards
        """
        assert 0 <= idx < 4

        return cls(cls.SourceType.REVEALED, cls.RevealedCardLocation(tier, idx), uses)

    @classmethod
    def from_reserved(cls, idx: int, uses: ColorVec):
        """
        create a BuyCardAction from the reserved cards
        """
        assert 0 <= idx < 3
        return cls(cls.SourceType.RESERVED, cls.ReservedCardLocation(idx), uses)

    def to_json(self) -> Dict:
        return {
            "type": "buy_card",
            "action": {
                "source": {
                    "type": self.source_type.value,
                    "location": self.location.to_json(),
                },
                "uses": self.uses.to_json(),
            },
        }


class DropTokensAction:
    """
    represent the action of dropping tokens
    """

    tokens: ColorVec

    def __init__(self, tokens: ColorVec):
        self.tokens = tokens

    def to_json(self) -> List[int]:
        return self.tokens.to_json()


class SelectNoblesAction:
    """
    represent the action of selecting nobles
    """

    idx: int

    def __init__(self, idx: int):
        assert 0 <= idx < 5
        self.idx = idx

    def to_json(self) -> int:
        return self.idx
