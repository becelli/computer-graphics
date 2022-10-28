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
        self.color_widget: QLabel = None
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
        self.canvas.setMouseTracking(True)
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
        self.display_current_pixel_info(event)
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

        self.display_current_pixel_info(event)

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
        # self.set_mouse_tracking_to_show_pixel_details(self.canvas)

        self.color_widget = QLabel()
        self.color_widget.setAlignment(Qt.AlignmentFlag.AlignCenter)
        self.color_widget.setFixedSize(150, 60)
        grid.addWidget(self.canvas, 1, 0)
        grid.addWidget(self.color_widget, 2, 0)

        grid.setRowStretch(3, 1)
        qto.display_grid_on_window(self, grid)

    def display_current_pixel_info(self, event: QMouseEvent):
        x, y = event.x(), event.y()
        image = qto.get_image_from_canvas(self.canvas)
        pixel_color = qto.get_pixel_color(image, x, y)
        r, g, b, a = pixel_color

        text_color = "white" if qto.is_color_dark(pixel_color) else "black"

        self.color_widget.setStyleSheet(
            f"""background-color: rgba({r}, {g}, {b}, {a});
            color: {text_color};
            border-radius: 10px;
            
            """
        )
        self.color_widget.setText(
            f"({ x }, {y})\n\n" f"rgba({r}, {g}, {b}, {a})")
