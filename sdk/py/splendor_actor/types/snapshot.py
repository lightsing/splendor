from enum import Enum
from typing import List, Dict, Any, Tuple, Union

from .card import Card, ReservedCardType, DevelopmentCards, CardPool
from .color import Color, ColorVec
from .noble import Noble
from .tier import Tier
from .player import Player


class GameSnapshot:
    """
    represent the game state
    """

    last_round: bool
    current_round: int
    current_player: int
    tokens: ColorVec
    card_pool: CardPool
    nobles: List[Noble]
    players: List[Player]

    def __init__(
        self,
        last_round: bool,
        current_round: int,
        current_player: int,
        tokens: List[int],
        card_pool: Dict,
        nobles: List[Dict],
        players: List[Dict],
    ):
        self.last_round = last_round
        self.current_round = current_round
        self.current_player = current_player
        self.tokens = ColorVec(tokens)
        self.card_pool = CardPool(**card_pool)
        self.nobles = [Noble(**noble) for noble in nobles]
        self.players = [Player(**player) for player in players]

    @staticmethod
    def from_json(json: Dict) -> "GameSnapshot":
        """
        Convert a json to a GameSnapshot
        """
        return GameSnapshot(**json)
