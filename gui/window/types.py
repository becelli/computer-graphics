from dataclasses import dataclass


class OPCODE:
    NONE = 0
    DRAW_LINE = 1
    DRAW_LINE_BRESENHAM = 2
    DRAW_CIRCLE = 3
    DRAW_CIRCLE_BRESENHAM = 4


@dataclass
class Point:
    x: int
    y: int
