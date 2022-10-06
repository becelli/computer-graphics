from PyQt5.QtWidgets import QLabel
import gui.qt_override as qto
from PyQt5.QtGui import QPixmap


def open_image(canvas: QLabel):
    filename = qto.QDialogs().get_open_path()
    if filename:
        pixmap = QPixmap(filename)
        qto.put_pixmap_on_canvas(canvas, pixmap)


def save_image(canvas: QLabel):
    filename = qto.QDialogs().get_save_path()
    if filename:
        qto.get_pixmap_from_canvas(canvas).save(filename)
