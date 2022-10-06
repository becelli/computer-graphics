from dataclasses import dataclass
from PyQt5.QtGui import QImage
import numpy as np
# import libkayn as kayn
import gui.qt_override as qto


@dataclass
class Operations:
    img: QImage

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
        return QImage(filtered, w, h, QImage.Format.Format_RGBA8888)

    def area_filter(self, function: callable, mask_side, **kwargs) -> QImage:
        w, h = self.img.width(), self.img.height()
        image = self.get_img_pixels(w, h)
        result = np.array(function(image, **kwargs),
                          dtype=np.uint8).astype(np.uint8)
        result = result.reshape(h, w, 4)

        new_w, new_h = w - mask_side + 1, h - mask_side + 1
        return QImage(result, new_w, new_h, QImage.Format.Format_RGBA8888)


class CG(Operations):
    def __init__(self, img: QImage):
        super().__init__(img)

    def apply_operation(self, filter: str):
        all_operations = {"Line...": lambda: f.grayscale()}
        image = qto.get_image_from_canvas(self.canvas)
        f = Operations(image)
        if filter in all_operations:
            output = all_operations[filter]()
            self.update_canvas(output)
        return
