from enum import Enum


class Tier(Enum):
    """
    Enum for the tiers of the cards
    """

    I = 0
    II = 1
    III = 2

    def __str__(self):
        return self.name

    def __repr__(self):
        return self.name

    def __lt__(self, other):
        return self.value < other.value
