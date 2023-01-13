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

    @staticmethod
    def get_img_pixels(image: QImage,  w, h):
        bits = np.array(image.bits().asarray(w * h * 4))
        pixels = bits.reshape(h, w, 4)  # Use matrix to represent the image
        image = pixels[:, :, [2, 1, 0, 3]]  # BGR -> RGB
        return image

    def create_new_image(self, width=320, height=240):
        return QImage(width, height, QImage.Format.Format_RGBA8888)

    def qcolor_to_rgb(self, color: QColor):
        return [color.red(), color.green(), color.blue(), color.alpha()]

    def get_default_elements_to_filters(self) -> tuple:
        w, h = self.img.width(), self.img.height()
        image = self.create_new_image(w, h)
        return w, h, image

    def default_filter(self, filter_func: callable, **kwargs) -> QImage:
        w, h = self.img.width(), self.img.height()
        image = Operations.get_img_pixels(self.img, w, h)

        filtered = np.array(filter_func(image, **kwargs),
                            dtype=np.uint8).astype(np.uint8)
        self.img = QImage(filtered, w, h, QImage.Format.Format_RGBA8888)
        return self.img

    def area_filter(self, function: callable, mask_side, **kwargs) -> QImage:
        w, h = self.img.width(), self.img.height()
        image = Operations.get_img_pixels(self.img, w, h)
        result = np.array(function(image, **kwargs),
                          dtype=np.uint8).astype(np.uint8)
        result = result.reshape(h, w, 4)

        new_w, new_h = w - mask_side + 1, h - mask_side + 1
        self.img = QImage(result, new_w, new_h, QImage.Format.Format_RGBA8888)
        return self.img

    def draw_line(self, **kwargs):
        p0, p1 = kwargs['points']
        color = self.qcolor_to_rgb(kwargs['color'])

        return self.default_filter(cglib.draw_line, p0=p0, p1=p1, color=color)

    def draw_line_bresenham(self, **kwargs):
        p0, p1 = kwargs['points']
        color = self.qcolor_to_rgb(kwargs['color'])

        selection_points = kwargs['selection_points']
        if len(selection_points) == 2:
            p_0, p_1 = selection_points
            p_0_, p_1_ = (p_0.x, p_0.y), (p_1.x, p_1.y)
            boundary = (p_0_, p_1_)

            return self.default_filter(cglib.cohen_sutherland, p0=p0, p1=p1, color=color, boundary=boundary)

        return self.default_filter(cglib.draw_line_bresenham, p0=p0, p1=p1, color=color)

    def draw_circle(self, **kwargs):
        p0, p1 = kwargs['points']
        color = self.qcolor_to_rgb(kwargs['color'])
        return self.default_filter(cglib.draw_circle, p0=p0, p1=p1, color=color)

    def draw_circle_bresenham(self, **kwargs):
        p0, p1 = kwargs['points']
        color = self.qcolor_to_rgb(kwargs['color'])
        return self.default_filter(cglib.draw_circle_bresenham, p0=p0, p1=p1, color=color)

    def draw_circle_parametric(self, **kwargs):
        p0, p1 = kwargs['points']
        color = self.qcolor_to_rgb(kwargs['color'])
        return self.default_filter(cglib.draw_circle_parametric, p0=p0, p1=p1, color=color)

    def draw_triangle(self, **kwargs):
        p0, p1, p2 = kwargs['points']
        color = self.qcolor_to_rgb(kwargs['color'])
        return self.default_filter(cglib.draw_triangle, p0=p0, p1=p1, p2=p2, color=color)

    def flood_fill(self, **kwargs):
        p0 = kwargs['point']
        neighbors = kwargs['neighbors']
        n4 = False if neighbors == 8 else True
        color = self.qcolor_to_rgb(kwargs['color'])
        return self.default_filter(cglib.flood_fill, p0=p0, color=color, n4=n4)

    @staticmethod
    def shear(image: QImage, edges: list, matrix: np.ndarray) -> tuple[QImage, list]:
        w, h = image.width(), image.height()

        image = Operations.get_img_pixels(image, w, h)

        image_result, edges_result = cglib.shear_object(
            image, edges=edges,  matrix=matrix)

        new_image = np.array(image_result, dtype=np.uint8).astype(np.uint8)
        img = QImage(new_image, w, h, QImage.Format.Format_RGBA8888)
        return img, edges_result

    @staticmethod
    def rotate(image: QImage, edges: list,  axis: str, angle: np.float64, aroundItself: bool) -> tuple[QImage, list]:
        w, h = image.width(), image.height()

        image = Operations.get_img_pixels(image, w, h)

        image_result, edges_result = cglib.rotate_object(
            image, edges=edges,  degrees=angle, axis=axis, center=aroundItself)
        new_image = np.array(image_result, dtype=np.uint8).astype(np.uint8)
        img = QImage(new_image, w, h, QImage.Format.Format_RGBA8888)
        return img, edges_result

    @staticmethod
    def translate(image: QImage, edges: list, axis: np.ndarray) -> tuple[QImage, list]:
        w, h = image.width(), image.height()

        image = Operations.get_img_pixels(image, w, h)

        image_result, edges_result = cglib.translate_object(
            image, edges=edges,  axis=axis)
        new_image = np.array(image_result, dtype=np.uint8).astype(np.uint8)
        img = QImage(new_image, w, h, QImage.Format.Format_RGBA8888)
        return img, edges_result

    @staticmethod
    def scale(image: QImage, edges: list, scale: np.ndarray) -> tuple[QImage, list]:
        w, h = image.width(), image.height()

        image = Operations.get_img_pixels(image, w, h)

        image_result, edges_result = cglib.scale_object(
            image, edges=edges,  scale=scale)
        new_image = np.array(image_result, dtype=np.uint8).astype(np.uint8)
        img = QImage(new_image, w, h, QImage.Format.Format_RGBA8888)
        return img, edges_result

    @staticmethod
    def get_objects(index=1):
        objects = cglib.get_object(index)

        return objects

    @staticmethod
    def print_objects_in_screen(image: QImage, edges: list):
        w, h = image.width(), image.height()

        image = Operations.get_img_pixels(image, w, h)

        image_result = cglib.print_objects_in_screen(image=image, points=edges)
        new_image = np.array(image_result, dtype=np.uint8).astype(np.uint8)
        img = QImage(new_image, w, h, QImage.Format.Format_RGBA8888)

        return img


class CG():
    def __init__(self, canvas: QLabel):
        self.canvas = canvas
        self.f = Operations(None)

    def apply(self, code: int, **kwargs):
        all_operations = {
            OPCODE.DRAW_LINE: self.f.draw_line,
            OPCODE.DRAW_LINE_BRESENHAM: self.f.draw_line_bresenham,
            OPCODE.DRAW_CIRCLE: self.f.draw_circle,
            OPCODE.DRAW_CIRCLE_BRESENHAM: self.f.draw_circle_bresenham,
            OPCODE.DRAW_CIRCLE_PARAMETRIC: self.f.draw_circle_parametric,
            OPCODE.DRAW_TRIANGLE: self.f.draw_triangle,
            OPCODE.FLOOD_FILL_4: self.f.flood_fill,
            OPCODE.FLOOD_FILL_8: self.f.flood_fill,
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
