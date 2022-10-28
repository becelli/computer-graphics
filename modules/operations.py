from dataclasses import dataclass
from lib2to3.pgen2.token import OP
from PyQt5.QtGui import QImage, QColor
from PyQt5.QtWidgets import QLabel
from PyQt5.QtCore import QPoint
import numpy as np
import cglib
from gui.window.types import OPCODE, Point
import gui.qt_override as qto


@dataclass
class Operations:
    img: QImage

    def set_img(self, img: QImage):
        self.img = img

    def get_img_pixels(self, w, h):
        bits = np.array(self.img.bits().asarray(w * h * 4))
        pixels = bits.reshape(h, w, 4)  # Use matrix to represent the image
        image = pixels[:, :, [2, 1, 0, 3]]  # BGR -> RGB
        return image

    def create_new_image(self, width=320, height=240):
        return QImage(width, height, QImage.Format.Format_RGBA8888)

    def get_default_elements_to_filters(self) -> tuple:
        w, h = self.img.width(), self.img.height()
        image = self.create_new_image(w, h)
        return w, h, image

    def default_filter(self, filter_func: callable, **kwargs) -> QImage:
        w, h = self.img.width(), self.img.height()
        image = self.get_img_pixels(w, h)

        filtered = np.array(filter_func(image, **kwargs),
                            dtype=np.uint8).astype(np.uint8)
        self.img = QImage(filtered, w, h, QImage.Format.Format_RGBA8888)
        return self.img

    def area_filter(self, function: callable, mask_side, **kwargs) -> QImage:
        w, h = self.img.width(), self.img.height()
        image = self.get_img_pixels(w, h)
        result = np.array(function(image, **kwargs),
                          dtype=np.uint8).astype(np.uint8)
        result = result.reshape(h, w, 4)

        new_w, new_h = w - mask_side + 1, h - mask_side + 1
        self.img = QImage(result, new_w, new_h, QImage.Format.Format_RGBA8888)
        return self.img

    def draw_line(self, points: list[Point]):
        p0, p1 = [p.to_tuple() for p in points]
        return self.default_filter(cglib.draw_line, p0=p0, p1=p1, color=[255, 0, 0, 255])

    def draw_line_bresenham(self, points: list[Point]):
        p0, p1 = [p.to_tuple() for p in points]
        return self.default_filter(cglib.draw_line_bresenham, p0=p0, p1=p1, color=[255, 0, 0, 255])

    def draw_circle(self, points: list[Point]):
        p0, p1 = points
        return self.default_filter(cglib.draw_circle, p0=p0, p1=p1, color=[255, 0, 0, 255])

    def draw_circle_bresenham(self, points: list[Point]):
        p0, p1 = points
        return self.default_filter(cglib.draw_circle_bresenham, p0=p0, p1=p1, color=[255, 0, 0, 255])


class CG():
    def __init__(self, canvas: QLabel):
        self.canvas = canvas
        self.f = Operations(None)

    def apply(self, code: int, **kwargs):
        all_operations = {OPCODE.DRAW_LINE: self.f.draw_line,
                          OPCODE.DRAW_LINE_BRESENHAM: self.f.draw_line_bresenham,
                          OPCODE.DRAW_CIRCLE: self.f.draw_circle,
                          OPCODE.DRAW_CIRCLE_BRESENHAM: self.f.draw_circle_bresenham,
                          }

        if code in all_operations:
            self.update_reference_image()
            output = all_operations[code](**kwargs)
            self.update_canvas(output)

    def update_reference_image(self):
        image = qto.get_image_from_canvas(self.canvas)
        self.f.set_img(image)

    def update_canvas(self, image: QImage):
        qto.put_image_on_canvas(self.canvas, image)
        return
