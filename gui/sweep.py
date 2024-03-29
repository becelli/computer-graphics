from PyQt5.QtWidgets import QLabel, QPushButton
from PyQt5.QtGui import QPixmap, QColor, QPen, QPainter, QMouseEvent
from gui.qt_override import QGrid, QChildWindow, display_grid_on_window
import gui.qt_override as qto
from modules.operations import Operations


class Sweep(QChildWindow):
    def __init__(self, parent):
        self.parent = parent

        self.input_canvas = QLabel()
        self.result_canvas = QLabel()
        self.initial_pixmap: QPixmap = None
        self.should_draw = False
        self.background_color = QColor(0, 0, 0)
        self.foreground_color = QColor(255, 255, 255)

        self.bar_color = QColor(128, 128, 128)

        self.show_content()
        self.enable_draw()

    def reset(self):

        initial_pixmap = QPixmap(self.initial_pixmap)
        # add a vertical line to the middle of the image
        painter = QPainter(initial_pixmap)
        painter.setPen(QPen(self.bar_color, 1))
        painter.drawLine(int(initial_pixmap.width() / 2), 0,
                         int(initial_pixmap.width() / 2), initial_pixmap.height())
        painter.end()

        self.input_canvas.setPixmap(initial_pixmap)
        self.result_canvas.setPixmap(self.initial_pixmap)

    def apply(self):

        image_canvas = qto.get_image_from_canvas(self.input_canvas)
        img = Operations.rotate_plane_sweep(
            image_canvas, self.foreground_color)
        self.result_canvas.setPixmap(QPixmap.fromImage(img))

    def enable_draw(self):
        pen = QPen()
        pen.setColor(self.foreground_color)
        pen.setWidth(1)

        self.input_canvas.setMouseTracking(True)
        self.input_canvas.mouseMoveEvent = lambda event: self.draw(event, pen)
        self.input_canvas.mousePressEvent = lambda _: setattr(
            self, "should_draw", True)
        self.input_canvas.mouseReleaseEvent = lambda _: setattr(
            self, "should_draw", False)

    def draw(self, event: QMouseEvent, pen: QPen = None) -> None:
        if not self.should_draw:
            return

        x, y = event.pos().x(), event.pos().y()
        pixmap = QPixmap(self.input_canvas.pixmap())
        w = pixmap.width()
        if x < w / 2:
            return

        painter = QPainter(pixmap)
        painter.setPen(pen)
        painter.drawPoint(x, y)
        painter.end()
        self.input_canvas.setPixmap(pixmap)

    def show_content(self) -> None:

        window = QChildWindow(self.parent, "Sweep")
        grid = QGrid()
        grid.setSpacing(10)

        # Canvas on left, controls on right
        w, h = 450, 450

        self.input_canvas = qto.create_canvas(w, h, self.background_color)
        self.result_canvas = qto.create_canvas(w, h, self.background_color)
        self.initial_pixmap = QPixmap(self.result_canvas.pixmap())
        self.reset()

        grid.addWidget(self.input_canvas, 0, 0, 4, 2)
        grid.addWidget(self.result_canvas, 0, 2, 4, 2)

        # Reset button
        reset_button = QPushButton("Reset")
        reset_button.clicked.connect(lambda: self.reset())
        # Expand button to fill the whole column
        grid.addWidget(reset_button, 5, 1)

        # Apply button
        apply_button = QPushButton("Apply")
        apply_button.clicked.connect(lambda: self.apply())
        # Expand button to fill the whole column

        grid.addWidget(apply_button, 5, 2)

        # grid.setColumnStretch(4, 1)
        # grid.setRowStretch(5, 1)

        # add padding
        grid.setContentsMargins(10, 10, 10, 10)

        display_grid_on_window(window, grid)
