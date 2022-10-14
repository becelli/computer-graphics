from dataclasses import dataclass
from lib2to3.pgen2.token import OP
from PyQt5.QtGui import QImage, QColor
from PyQt5.QtWidgets import QLabel
from PyQt5.QtCore import QPoint
import numpy as np
# import libkayn as kayn
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

    def draw_line(self, p0: Point, p1: Point):
        import random

        print(p0, p1)
        w, h = self.img.width(), self.img.height()

        for i in range(100):
            x1, y1 = random.randint(0, w), random.randint(0, h)
            x2, y2 = random.randint(0, w), random.randint(0, h)
            rgb = QColor(200, 255, 255)
            self.img.setPixel(x1, y1, rgb.rgba())
            self.img.setPixel(x2, y2, rgb.rgba())

        return self.img


class CG():
    def __init__(self, canvas: QLabel):
        self.canvas = canvas
        self.f = Operations(None)

    def apply(self, code: int, **kwargs):
        all_operations = {OPCODE.DRAW_LINE: self.f.draw_line}

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
