from PyQt5.QtWidgets import QLabel, QLineEdit, QRadioButton, QPushButton
from PyQt5.QtGui import QFont, QRegExpValidator, QPixmap
from PyQt5.QtCore import Qt, QRegExp
from gui.qt_override import QGrid, QChildWindow, display_grid_on_window
import numpy as np
import gui.qt_override as qto
from modules.operations import Operations


class ZBuffer(QChildWindow):
    def __init__(self, parent):
        self.parent = parent

        self.original_edges = []
        self.new_edges = self.original_edges.copy()

        self.translations = np.array([0.0, 0.0, 0.0], dtype=np.float64)
        self.rotation_axis: str = "x"
        self.rotation_angle: np.float64 = np.float64(0.0)
        self.self_center: bool = False

        self.canvas = QLabel()
        self.backup_pixmap: QPixmap = None
        # can start with negative values
        self.float_validator = QRegExpValidator(QRegExp(r"[-]?\d*\.?\d*"))
        self.axis_validator = QRegExpValidator(QRegExp("[xXyYzZ]"))

        # Radio buttons
        self.radios = {
            "LOCAL_SCALE": QRadioButton("Local"),
            "GLOBAL_SCALE": QRadioButton("Global"),
            "TRANSLATE": QRadioButton("Translate"),
            "ORIGIN_ROTATE": QRadioButton("Origin of canvas"),
            "CENTER_ROTATE": QRadioButton("Center of object"),
            "SHEAR": QRadioButton("Shear"),
        }

        self.show_content()

    def reset(self):
        self.new_edges = self.original_edges.copy()
        self.canvas.setPixmap(self.backup_pixmap)

        image_canvas = qto.get_image_from_canvas(self.canvas)

        # Shear takes a 4x4 matrix. We'll use the identity matrix for the reset
        edges = Operations.get_objects()
        img = Operations.print_objects_in_screen(image_canvas, edges)

        self.canvas.setPixmap(QPixmap.fromImage(img))
        self.new_edges = edges

    def apply(self):
        selected_radio = None
        for radio in self.radios.values():
            if radio.isChecked():
                selected_radio = radio
                break

        if selected_radio is None:
            return

        self.canvas.setPixmap(self.backup_pixmap)
        image_canvas = qto.get_image_from_canvas(self.canvas)
        if selected_radio == self.radios["ORIGIN_ROTATE"]:
            img, edges = Operations.rotate(
                image_canvas, self.new_edges, self.rotation_axis, self.rotation_angle, False)
        elif selected_radio == self.radios["CENTER_ROTATE"]:
            img, edges = Operations.rotate(
                image_canvas, self.new_edges, self.rotation_axis, self.rotation_angle, True)
        elif selected_radio == self.radios["TRANSLATE"]:
            img, edges = Operations.translate(
                image_canvas, self.new_edges, self.translations)
        else:
            return

        self.canvas.setPixmap(QPixmap.fromImage(img))
        self.new_edges = edges

    def set_radio_selected(self, radio_name: str):
        for radio in self.radios.values():
            radio.setChecked(False)
        self.radios[radio_name].setChecked(True)

    def set_scale(self, axis: str, value: np.float64):
        try:
            value = np.float64(value)
            self.scales[["x", "y", "z", "w"].index(axis)] = value
        except ValueError:
            pass

    def set_translation(self, axis: str, value: np.float64):
        try:
            value = np.float64(value)
            self.translations[["x", "y", "z"].index(axis)] = value
        except ValueError:
            pass

    def set_rotation_axis(self, axis: str):
        self.rotation_axis = axis.lower()

    def set_rotation_angle(self, angle: np.float64):
        try:
            self.rotation_angle = np.float64(angle)
        except ValueError:
            pass

    def show_content(self) -> None:

        window = QChildWindow(self.parent, "ZBuffer")
        grid = QGrid()
        grid.setSpacing(10)

        # Canvas on left, controls on right
        w, h = 720, 720
        self.canvas = qto.create_canvas(w, h)
        self.backup_pixmap = QPixmap(self.canvas.pixmap())
        self.reset()
        grid.addWidget(self.canvas, 0, 0, 6, 1)

        # Translation
        translation_label = QLabel("Translation")
        translation_label.setFont(QFont("Arial", 12, QFont.Bold))
        translation_label.setAlignment(Qt.AlignCenter)
        grid.addWidget(translation_label, 0, 1)

        # X translation
        x_label = QLabel("X")
        x_label.setAlignment(Qt.AlignCenter)
        x_input_t = QLineEdit()
        x_input_t.setAlignment(Qt.AlignCenter)
        x_input_t.setValidator(self.float_validator)
        x_input_t.setText(str(self.translations[0]))
        x_input_t.textEdited.connect(
            lambda: self.set_translation("x", x_input_t.text()))
        grid.addWidget(x_label, 0, 2)
        grid.addWidget(x_input_t, 1, 2)

        # Y translation
        y_label = QLabel("Y")
        y_label.setAlignment(Qt.AlignCenter)
        y_input_t = QLineEdit()
        y_input_t.setAlignment(Qt.AlignCenter)
        y_input_t.setValidator(self.float_validator)
        y_input_t.setText(str(self.translations[1]))
        y_input_t.textEdited.connect(
            lambda: self.set_translation("y", y_input_t.text()))
        grid.addWidget(y_label, 0, 3)
        grid.addWidget(y_input_t, 1, 3)

        # Z translation
        z_label = QLabel("Z")
        z_label.setAlignment(Qt.AlignCenter)
        z_input_t = QLineEdit()
        z_input_t.setAlignment(Qt.AlignCenter)
        z_input_t.setValidator(self.float_validator)
        z_input_t.setText(str(self.translations[2]))
        z_input_t.textEdited.connect(
            lambda: self.set_translation("z", z_input_t.text()))
        grid.addWidget(z_label, 0, 4)
        grid.addWidget(z_input_t, 1, 4)

        self.radios["TRANSLATE"].clicked.connect(
            lambda: self.set_radio_selected("TRANSLATE"))
        grid.addWidget(self.radios["TRANSLATE"], 1, 1)

        # Rotation
        rotation_label = QLabel("Rotation")
        rotation_label.setFont(QFont("Arial", 12, QFont.Bold))
        rotation_label.setAlignment(Qt.AlignCenter)
        grid.addWidget(rotation_label, 2, 1)

        # Origin radio
        self.radios["ORIGIN_ROTATE"].clicked.connect(
            lambda: self.set_radio_selected("ORIGIN_ROTATE"))
        self.radios["CENTER_ROTATE"].clicked.connect(
            lambda: self.set_radio_selected("CENTER_ROTATE"))

        grid.addWidget(self.radios["ORIGIN_ROTATE"], 3, 1)
        grid.addWidget(self.radios["CENTER_ROTATE"], 4, 1)

        # Axis
        axis_label = QLabel("Axis")
        axis_label.setAlignment(Qt.AlignCenter)
        axis_input = QLineEdit()
        axis_input.setAlignment(Qt.AlignCenter)
        axis_input.setValidator(self.axis_validator)
        axis_input.setText(self.rotation_axis)
        axis_input.textEdited.connect(
            lambda: self.set_rotation_axis(axis_input.text()))
        axis_input.textEdited.emit(axis_input.text())
        grid.addWidget(axis_label, 3, 2)
        grid.addWidget(axis_input, 3, 3)

        # Angle
        angle_label = QLabel("Angle (deg)")
        angle_label.setAlignment(Qt.AlignCenter)
        angle_input = QLineEdit()
        angle_input.setAlignment(Qt.AlignCenter)
        angle_input.setValidator(self.float_validator)
        angle_input.setText(str(self.rotation_angle))
        angle_input.textEdited.connect(
            lambda: self.set_rotation_angle(angle_input.text()))
        grid.addWidget(angle_label, 4, 2)
        grid.addWidget(angle_input, 4, 3)

        # Reset button
        reset_button = QPushButton("Reset")
        reset_button.clicked.connect(lambda: self.reset())
        grid.addWidget(reset_button, 5, 1, 1, 2)

        # Apply button
        apply_button = QPushButton("Apply")
        apply_button.clicked.connect(lambda: self.apply())
        grid.addWidget(apply_button, 5, 3, 1, 2)

        grid.setRowStretch(5, 1)
        grid.setColumnStretch(4, 1)

        # add padding
        grid.setContentsMargins(20, 20, 20, 20)

        display_grid_on_window(window, grid)
