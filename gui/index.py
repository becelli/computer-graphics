import numpy as np
from PyQt5.QtWidgets import QApplication, QLabel, QMainWindow, QPushButton, QMenu
from PyQt5.QtGui import QPixmap, QImage, QFont, QGuiApplication, QMouseEvent, QIcon, QPainter, QPen, QBrush, QColor
from PyQt5.QtCore import Qt
from modules.operations import CG
import gui.colors_adapter as c_adpt
import gui.qt_override as qto
from gui.window import setup as w_setup, menubar as w_menubar
from gui.window.types import OPCODE, Point

TWO_POINTS_OPERATIONS = [OPCODE.DRAW_LINE, OPCODE.DRAW_CIRCLE,
                         OPCODE.DRAW_LINE_BRESENHAM, OPCODE.DRAW_CIRCLE_BRESENHAM]


class Application(QMainWindow):
    def __init__(self):
        super().__init__()
        # self.window_dimensions = (853, 480)
        self.canvas: QLabel = None
        self.operation: int = OPCODE.NONE
        self.buttons: list[QPushButton] = []
        self.points: list[Point] = []
        self.CGLIB = None
        self.setup()

    def setup(self):
        w_setup.set_window_properties(self)
        w_menubar.display_menubar(self)
        self.display_main_content()
        self.display_toolbar()
        self.listen_to_mouse_events()

    def set_operation(self, operation: int):
        self.operation = operation
    # track mouse movement to show pixel details

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

    # def set_mouse_tracking_on_click(self, element: QLabel) -> None:
    #     element.setMouseTracking(True)
    #     element.mousePressEvent = self.get_pixel_coordinates_and_color

    def update_canvas(self, new_image: QImage):
        if new_image is not None:
            qto.put_image_on_canvas(self.canvas, new_image)

    def display_toolbar(self):
        toolbar = self.addToolBar("Toolbar")
        toolbar.setMovable(False)
        toolbar.setFloatable(False)

        none_button = QPushButton()
        none_button.setIcon(QIcon("icons/cursor-fill.svg"))
        none_button.setToolTip("None")
        none_button.clicked.connect(
            lambda: self.select_button(none_button, OPCODE.NONE))
        self.buttons.append(none_button)
        toolbar.addWidget(none_button)

        lineButton = QPushButton()
        lineButton.setIcon(QIcon("icons/line.svg"))
        lineButton.setToolTip("Draw a Simple Line")
        lineButton.clicked.connect(
            lambda: self.select_button(lineButton, OPCODE.DRAW_LINE))
        self.buttons.append(lineButton)
        toolbar.addWidget(lineButton)

        lineBresenhamButton = QPushButton()
        lineBresenhamButton.setIcon(QIcon("icons/line-full.svg"))
        lineBresenhamButton.setToolTip("Bresenham's Line Algorithm")
        lineBresenhamButton.clicked.connect(
            lambda: self.select_button(lineBresenhamButton, OPCODE.DRAW_LINE_BRESENHAM))
        self.buttons.append(lineBresenhamButton)
        toolbar.addWidget(lineBresenhamButton)

        circleButton = QPushButton()
        circleButton.setIcon(QIcon("icons/circle.svg"))
        circleButton.setToolTip("Draw a Simple Circle")
        circleButton.clicked.connect(
            lambda: self.select_button(circleButton, OPCODE.DRAW_CIRCLE))
        self.buttons.append(circleButton)
        toolbar.addWidget(circleButton)

        circleBresenhamButton = QPushButton()
        circleBresenhamButton.setIcon(QIcon("icons/circle-full.svg"))
        circleBresenhamButton.setToolTip("Bresenham's Circle Algorithm")
        circleBresenhamButton.clicked.connect(
            lambda: self.select_button(circleBresenhamButton, OPCODE.DRAW_CIRCLE_BRESENHAM))
        self.buttons.append(circleBresenhamButton)
        toolbar.addWidget(circleBresenhamButton)

    def listen_to_mouse_events(self):
        self.canvas.mousePressEvent = self.mouse_click_event
        self.canvas.mouseMoveEvent = self.mouse_move_event

    def mouse_click_event(self, event: QMouseEvent):
        if self.operation == OPCODE.NONE:
            self.points = []
            return
        if self.operation in TWO_POINTS_OPERATIONS:
            if len(self.points) < 2:
                point = Point(event.x(), event.y())
                self.points.append(point)
                if len(self.points) == 1:
                    self.backup_image = self.canvas.pixmap().toImage()
                if len(self.points) == 2:
                    self.canvas.setPixmap(QPixmap.fromImage(self.backup_image))
                    self.CGLIB.apply(self.operation, points=self.points)
                    self.points = []

    def mouse_move_event(self, event: QMouseEvent):
        if self.operation == OPCODE.NONE:
            return
        if self.operation == OPCODE.DRAW_LINE or self.operation == OPCODE.DRAW_LINE_BRESENHAM:
            if len(self.points) == 1:
                self.canvas.setPixmap(QPixmap.fromImage(self.backup_image))
                pen = QPen()
                pen.setWidth(1)
                pen.setColor(QColor(255, 0, 0))
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                painter.drawLine(self.points[0].x, self.points[0].y,
                                 event.x(), event.y())
                painter.end()
                self.canvas.repaint()
            return
        if self.operation == OPCODE.DRAW_CIRCLE or self.operation == OPCODE.DRAW_CIRCLE_BRESENHAM:
            if len(self.points) == 1:
                self.canvas.setPixmap(QPixmap.fromImage(self.backup_image))
                pen = QPen()
                pen.setWidth(1)
                pen.setColor(QColor(255, 0, 0))
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                # Draw a circle with center at the first point and radius
                # equal to the distance between the first point and the current
                # mouse position
                center = self.points[0]
                radius = int(np.sqrt((center.x - event.x())**2 +
                                     (center.y - event.y())**2))
                radius = int(radius)
                painter.drawEllipse(center.x - radius, center.y -
                                    radius, radius * 2, radius * 2)
                painter.end()
                self.canvas.repaint()

    def select_button(self, button: QPushButton, opcode: int):
        for b in self.buttons:
            b.setDown(False)
        button.setDown(True)
        self.set_operation(opcode)
        self.points = []

    def display_main_content(self):
        grid = qto.QGrid()

        self.canvas = qto.create_canvas()
        self.backup_image = self.canvas.pixmap().toImage()
        self.CGLIB = CG(self.canvas)
        self.set_mouse_tracking_to_show_pixel_details(self.canvas)

        self.pixel_color_label = self.create_pixel_color_and_coordinates_widget()
        self.pixel_color_label.setFont(QFont("Monospace", pointSize=10))
        grid.addWidget(self.canvas, 1, 0)
        grid.addWidget(self.pixel_color_label, 2, 0)

        grid.setRowStretch(3, 1)
        qto.display_grid_on_window(self, grid)
