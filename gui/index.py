from PyQt5.QtWidgets import QApplication, QLabel, QMainWindow, QPushButton
from PyQt5.QtGui import QPixmap, QImage, QFont, QGuiApplication, QMouseEvent
from PyQt5.QtCore import Qt
from modules.operations import Operations
import gui.colors_adapter as c_adpt
import gui.qt_override as qto
from gui.window import setup as w_setup, menubar as w_menubar


class Application(QMainWindow):
    def __init__(self):
        super().__init__()
        # self.window_dimensions = (853, 480)
        self.canvas: QLabel = None
        self.operation: int = 0
        self.setup()

    def setup(self) -> None:
        w_setup.set_window_properties(self)
        w_menubar.display_menubar(self)
        self.display_main_content()

    def set_mouse_tracking_to_show_pixel_details(self, element: QLabel) -> None:
        element.setMouseTracking(True)
        def inform_canvas(
            e): return self.display_pixel_color_and_coordinates(e, element)
        element.mouseMoveEvent = inform_canvas

    def create_pixel_color_and_coordinates_widget(self) -> QLabel:
        widget = QLabel()
        widget.setAlignment(Qt.AlignmentFlag.AlignCenter)
        widget.setFixedSize(120, 60)
        return widget

    def display_pixel_color_and_coordinates(self, event: QMouseEvent, canvas: QLabel):
        x, y, color = self.get_pixel_coordinates_and_color(event, canvas)
        self.paint_new_pixel_color(color)
        self.display_new_pixel_color_info(x, y, color)

    def display_new_pixel_color_info(self, x: int, y: int, color):
        self.pixel_color_label.setText(
            f"({ x }, {y})\n\n" f"rgb({color[0]}, {color[1]}, {color[2]})")

    def paint_new_pixel_color(self, color: tuple[int, int, int]):
        r, g, b = color
        text_color = self.get_contrast_color(color)
        self.pixel_color_label.setStyleSheet(
            f"background-color: rgb({r}, {g}, {b});"
            f"color: {text_color};"
            f"border: 1px solid transparent;"
            f"border-radius: 10px;"
        )

    def get_contrast_color(self, bg_color: tuple[int, int, int]) -> str:
        return "white" if c_adpt.get_gray_from_rgb(*bg_color) < 128 else "black"

    def get_pixel_coordinates_and_color(
        self, event: QMouseEvent, canvas: QLabel = None
    ) -> tuple[int, int, tuple]:
        if canvas is None:
            canvas = self.canvas

        x, y = event.x(), event.y()
        image = qto.get_image_from_canvas(canvas)
        pixel_integer = image.pixel(x, y)
        color = c_adpt.get_rgb_from_color_integer(pixel_integer)
        return x, y, color

    def update_canvas(self, new_image: QImage):
        if new_image is not None:
            qto.put_image_on_canvas(self.canvas, new_image)

    def display_main_content(self):
        grid = qto.QGrid()

        self.canvas = qto.create_canvas()
        self.set_mouse_tracking_to_show_pixel_details(self.canvas)

        self.pixel_color_label = self.create_pixel_color_and_coordinates_widget()
        self.pixel_color_label.setFont(QFont("Monospace", pointSize=10))
        grid.addWidget(self.canvas, 1, 0)
        grid.addWidget(self.pixel_color_label, 2, 0)

        grid.setRowStretch(3, 1)
        qto.display_grid_on_window(self, grid)
