from typing import List

from .color import ColorVec


class Noble:
    """
    represent the noble cards
    """

    requires: ColorVec

    def __init__(self, requires: List[int]):
        self.requires = ColorVec(requires)
