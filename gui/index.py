import numpy as np


from random import randint as rdint
from PyQt5.QtWidgets import QApplication, QLabel, QMainWindow, QPushButton, QMenu, QColorDialog, QWidget, QSizePolicy
from PyQt5.QtGui import QPixmap, QImage, QFont, QGuiApplication, QMouseEvent, QIcon, QPainter, QPen, QBrush, QColor
from PyQt5.QtCore import Qt
from modules.operations import CG
import gui.qt_override as qto
from gui.window import setup as w_setup, menubar as w_menubar
from gui.window.types import OPCODE, Point

TWO_POINTS_OPERATIONS = [OPCODE.DRAW_LINE, OPCODE.DRAW_CIRCLE,
                         OPCODE.DRAW_LINE_BRESENHAM, OPCODE.DRAW_CIRCLE_BRESENHAM, OPCODE.DRAW_CIRCLE_PARAMETRIC]

LINE_OPERATIONS = [OPCODE.DRAW_LINE, OPCODE.DRAW_LINE_BRESENHAM]


class Application(QMainWindow):
    def __init__(self):
        super().__init__()
        self.canvas: QLabel = None
        self.backup_pixmap: QPixmap = None
        self.operation: int = OPCODE.NONE
        self.buttons: list[QPushButton] = []
        self.current_color_widget: QLabel = None
        self.points: list[Point] = []
        self.selection_points: list[Point] = []
        self.primary_color = QColor(
            rdint(0, 255), rdint(0, 255), rdint(0, 255))
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

        self.color_selector = QPushButton()
        self.color_selector.setIcon(QIcon("icons/color-picker.svg"))
        self.color_selector.setToolTip("Color Selector")
        self.color_selector.clicked.connect(self.display_system_color_selector)
        self.color_selector.setStyleSheet(
            f"background-color: {self.primary_color.name()};")

        self.buttons.append(self.color_selector)
        toolbar.addWidget(self.color_selector)

        line_button = QPushButton()
        line_button.setIcon(QIcon("icons/line.svg"))
        line_button.setToolTip("Draw a Simple Line")
        line_button.clicked.connect(
            lambda: self.select_button(line_button, OPCODE.DRAW_LINE))
        self.buttons.append(line_button)
        toolbar.addWidget(line_button)

        line_bresenham_button = QPushButton()
        line_bresenham_button.setIcon(QIcon("icons/line-full.svg"))
        line_bresenham_button.setToolTip("Bresenham's Line Algorithm")
        line_bresenham_button.clicked.connect(
            lambda: self.select_button(line_bresenham_button, OPCODE.DRAW_LINE_BRESENHAM))
        self.buttons.append(line_bresenham_button)
        toolbar.addWidget(line_bresenham_button)

        circle_button = QPushButton()
        circle_button.setIcon(QIcon("icons/circle.svg"))
        circle_button.setToolTip("Draw a Simple Circle")
        circle_button.clicked.connect(
            lambda: self.select_button(circle_button, OPCODE.DRAW_CIRCLE))
        self.buttons.append(circle_button)
        toolbar.addWidget(circle_button)

        circle_bresenham_button = QPushButton()
        circle_bresenham_button.setIcon(QIcon("icons/circle-full.svg"))
        circle_bresenham_button.setToolTip("Bresenham's Circle Algorithm")
        circle_bresenham_button.clicked.connect(
            lambda: self.select_button(circle_bresenham_button, OPCODE.DRAW_CIRCLE_BRESENHAM))
        self.buttons.append(circle_bresenham_button)
        toolbar.addWidget(circle_bresenham_button)

        circle_parametric_button = QPushButton()
        circle_parametric_button.setIcon(QIcon("icons/circle-parametric.svg"))
        circle_parametric_button.setToolTip("Parametric Circle Algorithm")
        circle_parametric_button.clicked.connect(
            lambda: self.select_button(circle_parametric_button, OPCODE.DRAW_CIRCLE_PARAMETRIC))
        self.buttons.append(circle_parametric_button)
        toolbar.addWidget(circle_parametric_button)

        triangle_button = QPushButton()
        triangle_button.setIcon(QIcon("icons/triangle.svg"))
        triangle_button.setToolTip("Draw a Simple Triangle")
        triangle_button.clicked.connect(
            lambda: self.select_button(triangle_button, OPCODE.DRAW_TRIANGLE))
        self.buttons.append(triangle_button)
        toolbar.addWidget(triangle_button)

        flood_fill_button = QPushButton()
        flood_fill_button.setIcon(QIcon("icons/flood-fill.svg"))
        flood_fill_button.setToolTip("Flood Fill Algorithm (4-Connected)")
        flood_fill_button.clicked.connect(
            lambda: self.select_button(flood_fill_button, OPCODE.FLOOD_FILL_4))
        self.buttons.append(flood_fill_button)
        toolbar.addWidget(flood_fill_button)

        flood_fill_8_button = QPushButton()
        flood_fill_8_button.setIcon(QIcon("icons/flood-fill-8.svg"))
        flood_fill_8_button.setToolTip("Flood Fill Algorithm (8-Connected)")
        flood_fill_8_button.clicked.connect(
            lambda: self.select_button(flood_fill_8_button, OPCODE.FLOOD_FILL_8))
        self.buttons.append(flood_fill_8_button)
        toolbar.addWidget(flood_fill_8_button)

        selection_area_button = QPushButton()
        selection_area_button.setIcon(QIcon("icons/selection.svg"))
        selection_area_button.setToolTip("Selection Area")
        selection_area_button.clicked.connect(
            lambda: self.select_button(selection_area_button, OPCODE.SELECTION_AREA))
        self.buttons.append(selection_area_button)
        toolbar.addWidget(selection_area_button)

        self.select_button(none_button, OPCODE.NONE)

    def display_system_color_selector(self):
        color = QColorDialog.getColor()
        if color.isValid():
            self.color_selector.setStyleSheet(
                f"background-color: {color.name()};")
            self.primary_color = color
            self.operation = OPCODE.NONE
            self.select_button(self.buttons[0], OPCODE.NONE)

    def listen_to_mouse_events(self):
        self.canvas.setMouseTracking(True)
        self.canvas.mouseReleaseEvent = self.mouse_release_event
        self.canvas.mousePressEvent = self.mouse_press_event
        self.canvas.mouseMoveEvent = self.mouse_move_event

    def mouse_press_event(self, event: QMouseEvent):

        if self.operation == OPCODE.NONE:
            self.points = []
            return

        if self.operation in TWO_POINTS_OPERATIONS:
            if len(self.points) == 0:
                self.canvas.setPixmap(self.backup_pixmap)

            if len(self.points) < 2:
                point = Point(event.x(), event.y())
                self.points.append(point)

                if len(self.points) == 2:
                    self.canvas.setPixmap(self.backup_pixmap)
                    self.CGLIB.apply(
                        self.operation, points=self.points, color=self.primary_color, selection_points=self.selection_points)
                    self.backup_pixmap = QPixmap(self.canvas.pixmap())
                    self.points = []

            return

        if self.operation == OPCODE.DRAW_TRIANGLE:
            if len(self.points) < 3:
                point = Point(event.x(), event.y())
                self.points.append(point)
                if len(self.points) == 1:
                    self.backup_pixmap = QPixmap(self.canvas.pixmap())
                if len(self.points) == 3:
                    self.canvas.setPixmap(self.backup_pixmap)
                    self.CGLIB.apply(
                        self.operation, points=self.points, color=self.primary_color)
                    self.backup_pixmap = QPixmap(self.canvas.pixmap())
                    self.points = []
            return

        if self.operation == OPCODE.FLOOD_FILL_4:
            point = Point(event.x(), event.y())
            self.CGLIB.apply(
                self.operation, point=point, color=self.primary_color, neighbors=4)
            self.backup_pixmap = QPixmap(self.canvas.pixmap())
            return

        if self.operation == OPCODE.FLOOD_FILL_8:
            point = Point(event.x(), event.y())
            self.CGLIB.apply(
                self.operation, point=point, color=self.primary_color, neighbors=8)
            self.backup_pixmap = QPixmap(self.canvas.pixmap())
            return

        if self.operation == OPCODE.SELECTION_AREA:
            if len(self.selection_points) == 0:
                point = Point(event.x(), event.y())
                self.selection_points.append(point)
                self.backup_pixmap = QPixmap(self.canvas.pixmap())

    def mouse_release_event(self, event: QMouseEvent):
        if self.operation == OPCODE.SELECTION_AREA:
            if len(self.selection_points) == 1:
                point = Point(event.x(), event.y())
                self.selection_points.append(point)
                self.canvas.setPixmap(self.backup_pixmap)

                # Reset the selection points if the user only clicked once
                x_ini, y_ini = self.selection_points[0].x, self.selection_points[0].y
                x_end, y_end = self.selection_points[1].x, self.selection_points[1].y
                if x_ini == x_end or y_ini == y_end:
                    self.selection_points = []
                    return

                # Draw the selection area
                pen = QPen(self.primary_color)
                pen.setWidth(1)
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                painter.drawRect(
                    self.selection_points[0].x, self.selection_points[0].y, self.selection_points[1].x - self.selection_points[0].x, self.selection_points[1].y - self.selection_points[0].y)
                painter.end()
                self.canvas.repaint()
                self.backup_pixmap = QPixmap(self.canvas.pixmap())

                self.select_button(self.buttons[0], OPCODE.NONE)

    def mouse_move_event(self, event: QMouseEvent):
        self.display_current_pixel_info(event)

        if self.operation == OPCODE.SELECTION_AREA:
            if len(self.selection_points) == 1:
                self.canvas.setPixmap(self.backup_pixmap)
                pen = QPen()
                pen.setColor(self.primary_color)
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                painter.drawRect(
                    self.selection_points[0].x, self.selection_points[0].y, event.x() - self.selection_points[0].x, event.y() - self.selection_points[0].y)
                painter.end()
                self.canvas.repaint()

        if self.operation == OPCODE.NONE:
            return
        if self.operation == OPCODE.DRAW_LINE or self.operation == OPCODE.DRAW_LINE_BRESENHAM:
            if len(self.points) == 1:
                self.canvas.setPixmap(self.backup_pixmap)
                pen = QPen()
                pen.setWidth(1)
                pen.setColor(self.primary_color)
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                painter.drawLine(self.points[0].x, self.points[0].y,
                                 event.x(), event.y())
                painter.end()
                self.canvas.repaint()
            return

        if self.operation == OPCODE.DRAW_CIRCLE or self.operation == OPCODE.DRAW_CIRCLE_BRESENHAM or OPCODE.DRAW_CIRCLE_PARAMETRIC == self.operation:
            if len(self.points) == 1:
                self.canvas.setPixmap(self.backup_pixmap)
                pen = QPen()
                pen.setWidth(1)
                pen.setColor(self.primary_color)
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                center = self.points[0]
                radius = int(np.sqrt((center.x - event.x())**2 +
                                     (center.y - event.y())**2))
                radius = int(radius)
                painter.drawEllipse(center.x - radius, center.y -
                                    radius, radius * 2, radius * 2)
                painter.end()
                self.canvas.repaint()
            return

        if self.operation == OPCODE.DRAW_TRIANGLE:
            if len(self.points) == 1:
                self.canvas.setPixmap(self.backup_pixmap)
                pen = QPen()
                pen.setWidth(1)
                pen.setColor(self.primary_color)
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                painter.drawLine(self.points[0].x, self.points[0].y,
                                 event.x(), event.y())
                painter.end()
                self.canvas.repaint()
            if len(self.points) == 2:
                self.canvas.setPixmap(self.backup_pixmap)
                pen = QPen()
                pen.setWidth(1)
                pen.setColor(self.primary_color)
                painter = QPainter(self.canvas.pixmap())
                painter.setPen(pen)
                painter.drawLine(self.points[0].x, self.points[0].y,
                                 event.x(), event.y())
                painter.drawLine(self.points[1].x, self.points[1].y,
                                 event.x(), event.y())
                painter.drawLine(self.points[0].x, self.points[0].y,
                                 self.points[1].x, self.points[1].y)
                painter.end()
                self.canvas.repaint()
            return

        self.display_current_pixel_info(event)

    def select_button(self, button: QPushButton, opcode: int):
        for b in self.buttons:
            b.setDown(False)
        button.setDown(True)
        self.set_operation(opcode)

        self.points = []

        if opcode == OPCODE.SELECTION_AREA:
            self.selection_points = []

    def display_main_content(self):
        grid = qto.QGrid()

        self.canvas = qto.create_canvas()
        grid.addWidget(self.canvas, 1, 0)
        self.CGLIB = CG(self.canvas)
        self.backup_pixmap = QPixmap(self.canvas.pixmap())

        self.current_color_widget = QLabel()
        self.current_color_widget.setAlignment(Qt.AlignmentFlag.AlignCenter)
        # widget should occupy the entire cell
        self.current_color_widget.setSizePolicy(
            QSizePolicy.Expanding, QSizePolicy.Expanding)
        self.current_color_widget.setStyleSheet(
            f"""background-color: rgba(0, 0, 0, 255);
            color: white;   
            padding-left: 10px;
            padding-right: 10px;
            padding-top: 5px;
            padding-bottom: 5px;
            """
        )

        self.current_color_widget.setText("xy(0, 0) — rgba(0, 0, 0, 255)")
        grid.addWidget(self.current_color_widget, 2, 0)

        grid.setRowStretch(2, 2)
        qto.display_grid_on_window(self, grid)

    def display_current_pixel_info(self, event: QMouseEvent):
        x, y = event.x(), event.y()
        image = qto.get_image_from_canvas(self.canvas)
        pixel_color = qto.get_pixel_color(image, x, y)
        r, g, b, a = pixel_color

        text_color = "white" if qto.is_color_dark(pixel_color) else "black"

        # Use the entire grid to display the current pixel info
        self.current_color_widget.setStyleSheet(
            f"""background-color: rgba({r}, {g}, {b}, {a});
            color: {text_color};
            padding-left: 10px;
            padding-right: 10px;
            padding-top: 5px;
            padding-bottom: 5px;
            """
        )
        self.current_color_widget.setText(
            f"xy({ x }, {y})" f' — '  f"rgba({r}, {g}, {b}, {a})")
