from typing import List, Dict, Optional

from .card import CardView, DevelopmentCards, Card
from .color import ColorVec, Color
from .noble import Noble


class Player:
    """
    represent the player
    """

    idx: int
    points: int
    tokens: ColorVec
    development_cards: DevelopmentCards
    reserved_cards: List[CardView]
    nobles: List[Noble]

    def __init__(
        self,
        idx: int,
        points: int,
        tokens: List[int],
        development_cards: Dict,
        reserved_cards: List[Dict],
        nobles: List[Dict],
    ):
        self.idx = idx
        self.points = points
        self.tokens = ColorVec(tokens)
        self.development_cards = DevelopmentCards(**development_cards)
        self.reserved_cards = [CardView(**card) for card in reserved_cards]
        self.nobles = [Noble(**noble) for noble in nobles]

    def can_buy(self, card: Card) -> Optional[ColorVec]:
        """
        check if the player can buy the card, return the tokens needed if possible
        """
        effective_cost = card.requires.saturating_sub(self.development_cards.bonus)
        diff = effective_cost.saturating_sub(self.tokens)
        if diff.total() > self.tokens[Color.YELLOW]:
            return None
        uses = effective_cost - diff
        uses[Color.YELLOW] = diff.total()
        return uses
