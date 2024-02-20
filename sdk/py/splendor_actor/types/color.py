from enum import Enum
from typing import List, Union


class Color(Enum):
    """
    Enum for the colors of the tokens and cards
    """

    BLACK = 0
    BLUE = 1
    GREEN = 2
    RED = 3
    WHITE = 4
    YELLOW = 5

    @staticmethod
    def from_str(s: str) -> "Color":
        """
        Convert a string to a Color enum
        """
        s = s.lower()
        if s == "black":
            return Color.BLACK
        elif s == "blue":
            return Color.BLUE
        elif s == "green":
            return Color.GREEN
        elif s == "red":
            return Color.RED
        elif s == "white":
            return Color.WHITE
        elif s == "yellow":
            return Color.YELLOW
        else:
            raise ValueError(f"Invalid color string: {s}")

    def __str__(self):
        return self.name

    def __repr__(self):
        return self.name

    def to_json(self) -> str:
        """
        Convert a Color to a json
        """
        return self.name.lower()


class ColorVec:
    """
    represent the color combinations
    """

    vec: List[int]

    def __init__(self, vec: List[int]):
        assert len(vec) == 6, "ColorVec should have 6 elements"

        self.vec = vec

    def __getitem__(self, index: Union[int, Color]) -> int:
        if isinstance(index, int):
            return self.vec[index]
        else:
            return self.vec[index.value]

    def __setitem__(self, index: Union[int, Color], value: int):
        assert value >= 0, "ColorVec should not contain negative values"

        if isinstance(index, int):
            self.vec[index] = value
        else:
            self.vec[index.value] = value

    def __lt__(self, other):
        return self.vec < other.vec

    def __le__(self, other):
        return self.vec <= other.vec

    def __gt__(self, other):
        return self.vec > other.vec

    def __ge__(self, other):
        return self.vec >= other.vec

    def __eq__(self, other):
        return self.vec == other.vec

    def __ne__(self, other):
        return self.vec != other.vec

    def __iter__(self):
        return ((Color(color), count) for color, count in enumerate(self.vec))

    def __add__(self, other):
        return ColorVec([a + b for a, b in zip(self.vec, other.vec)])

    def __sub__(self, other):
        return ColorVec([a - b for a, b in zip(self.vec, other.vec)])

    def saturating_sub(self, other):
        """
        Subtract other from self, but never go below 0
        """
        return ColorVec([max(a - b, 0) for a, b in zip(self.vec, other.vec)])

    def total(self):
        """
        Return the total number of tokens in the vector
        """
        return sum(self.vec)

    @classmethod
    def empty(cls) -> "ColorVec":
        """
        Return an empty color vector
        """
        return cls([0, 0, 0, 0, 0, 0])

    def to_json(self) -> List[int]:
        """
        Convert a ColorVec to a json
        """
        return self.vec
