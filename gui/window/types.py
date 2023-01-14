from dataclasses import dataclass


class OPCODE:
    NONE = 0
    DRAW_LINE = 1
    DRAW_LINE_BRESENHAM = 2
    DRAW_CIRCLE = 3
    DRAW_CIRCLE_BRESENHAM = 4
    DRAW_CIRCLE_PARAMETRIC = 5
    DRAW_TRIANGLE = 6
    SELECTION_AREA = 7
    FLOOD_FILL_4 = 8
    FLOOD_FILL_8 = 9
    EDGE_FILL = 10


@dataclass
class Point(tuple):
    x: int
    y: int

    def __new__(cls, x, y):
        return super(Point, cls).__new__(cls, (x, y))

    def __repr__(self):
        return f'Point({self.x}, {self.y})'

    def to_tuple(self):
        return self.x, self.y
