from PyQt5.QtWidgets import QLabel, QLineEdit, QRadioButton, QPushButton, QSizePolicy
from PyQt5.QtGui import QFont, QRegExpValidator, QPixmap, QColor
from PyQt5.QtCore import Qt, QRegExp
from gui.qt_override import QGrid, QChildWindow, display_grid_on_window
import numpy as np
import gui.qt_override as qto
from modules.operations import Operations


class Illumination(QChildWindow):
    def __init__(self, parent):
        self.parent = parent

        self.canvas = QLabel()
        self.backup_pixmap: QPixmap = None
        # can start with negative values
        self.float_validator = QRegExpValidator(QRegExp(r"[-]?\d*\.?\d*"))
        self.axis_validator = QRegExpValidator(QRegExp("[xXyYzZ]"))

        self.kd_1: np.float64 = np.float64(1)
        self.ks_1: np.float64 = np.float64(1)
        self.kd_2: np.float64 = np.float64(1)
        self.ks_2: np.float64 = np.float64(1)
        self.k: np.float64 = np.float64(1)
        self.ia: np.float64 = np.float64(1)
        self.ka: np.float64 = np.float64(1)
        self.il: np.float64 = np.float64(1)
        self.n: np.float64 = np.float64(1)

        # Radio buttons
        self.radios = {
            "MODEL_1": QRadioButton("Model 1"),
            "MODEL_2": QRadioButton("Model 2"),
        }
        self.show_content()

    def reset(self):
        self.canvas.setPixmap(self.backup_pixmap)
        image_canvas = qto.get_image_from_canvas(self.canvas)

        img = Operations.apply_luminosity(
            image_canvas, True, kd_1=self.kd_1, ks_1=self.ks_1, kd_2=self.kd_2, ks_2=self.ks_2, k=self.k, ia=self.ia, ka=self.ka, il=self.il, n=self.n)

        self.canvas.setPixmap(QPixmap.fromImage(img))

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
        model = True if selected_radio.text() == "Model 1" else False
        img = Operations.apply_luminosity(
            image_canvas, model=model, kd_1=self.kd_1, ks_1=self.ks_1, kd_2=self.kd_2, ks_2=self.ks_2, k=self.k, ia=self.ia, ka=self.ka, il=self.il, n=self.n)

        self.canvas.setPixmap(QPixmap.fromImage(img))

    def set_radio_selected(self, radio_name: str):
        for radio in self.radios.values():
            radio.setChecked(False)
        self.radios[radio_name].setChecked(True)

    def set_variable(self, variable_name: str, value: str):
        try:
            self.__setattr__(variable_name, np.float64(value))
        except ValueError:
            pass

    def switch_image(self):
        self.use_ramp = not self.use_ramp
        self.reset()

    def show_content(self) -> None:

        window = QChildWindow(self.parent, "Illumination")
        grid = QGrid()
        grid.setSpacing(10)

        # Canvas on left, controls on right
        w, h = 500, 500

        self.canvas = qto.create_canvas(w, h, QColor(0, 0, 0))
        self.backup_pixmap = QPixmap(self.canvas.pixmap())
        self.reset()
        grid.addWidget(self.canvas, 0, 0, 5, 4)

        # Origin radio
        self.radios["MODEL_1"].clicked.connect(
            lambda: self.set_radio_selected("MODEL_1"))
        self.radios["MODEL_2"].clicked.connect(
            lambda: self.set_radio_selected("MODEL_2"))

        self.radios["MODEL_1"].setChecked(True)

        grid.addWidget(self.radios["MODEL_1"], 5, 0)
        grid.addWidget(self.radios["MODEL_2"], 5, 1)

        kd_1_label = QLabel("Kd 1")
        kd_1_label.setAlignment(Qt.AlignCenter)
        kd_1_input = QLineEdit()
        kd_1_input.setAlignment(Qt.AlignCenter)
        kd_1_input.setValidator(self.float_validator)
        kd_1_input.setText(str(self.kd_1))
        kd_1_input.textEdited.connect(
            lambda: self.set_variable("kd_1", kd_1_input.text()))
        grid.addWidget(kd_1_label, 5, 2)
        grid.addWidget(kd_1_input, 5, 3)

        ks_1_label = QLabel("Ks 1")
        ks_1_label.setAlignment(Qt.AlignCenter)
        ks_1_input = QLineEdit()
        ks_1_input.setAlignment(Qt.AlignCenter)
        ks_1_input.setValidator(self.float_validator)
        ks_1_input.setText(str(self.ks_1))
        ks_1_input.textEdited.connect(
            lambda: self.set_variable("ks_1", ks_1_input.text()))
        grid.addWidget(ks_1_label, 6, 2)
        grid.addWidget(ks_1_input, 6, 3)

        kd_2_label = QLabel("Kd 2")
        kd_2_label.setAlignment(Qt.AlignCenter)
        kd_2_input = QLineEdit()
        kd_2_input.setAlignment(Qt.AlignCenter)
        kd_2_input.setValidator(self.float_validator)
        kd_2_input.setText(str(self.kd_2))
        kd_2_input.textEdited.connect(
            lambda: self.set_variable("kd_2", kd_2_input.text()))
        grid.addWidget(kd_2_label, 7, 2)
        grid.addWidget(kd_2_input, 7, 3)

        ks_2_label = QLabel("Ks 2")
        ks_2_label.setAlignment(Qt.AlignCenter)
        ks_2_input = QLineEdit()
        ks_2_input.setAlignment(Qt.AlignCenter)
        ks_2_input.setValidator(self.float_validator)
        ks_2_input.setText(str(self.ks_2))
        ks_2_input.textEdited.connect(
            lambda: self.set_variable("ks_2", ks_2_input.text()))
        grid.addWidget(ks_2_label, 8, 2)
        grid.addWidget(ks_2_input, 8, 3)

        k_label = QLabel("K")
        k_label.setAlignment(Qt.AlignCenter)
        k_input = QLineEdit()
        k_input.setAlignment(Qt.AlignCenter)
        k_input.setValidator(self.float_validator)
        k_input.setText(str(self.k))
        k_input.textEdited.connect(
            lambda: self.set_variable("k", k_input.text()))
        grid.addWidget(k_label, 9, 2)
        grid.addWidget(k_input, 9, 3)

        ia_label = QLabel("Ia")
        ia_label.setAlignment(Qt.AlignCenter)
        ia_input = QLineEdit()
        ia_input.setAlignment(Qt.AlignCenter)
        ia_input.setValidator(self.float_validator)
        ia_input.setText(str(self.ia))
        ia_input.textEdited.connect(
            lambda: self.set_variable("ia", ia_input.text()))
        grid.addWidget(ia_label, 10, 2)
        grid.addWidget(ia_input, 10, 3)

        ka_label = QLabel("Ka")
        ka_label.setAlignment(Qt.AlignCenter)
        ka_input = QLineEdit()
        ka_input.setAlignment(Qt.AlignCenter)
        ka_input.setValidator(self.float_validator)
        ka_input.setText(str(self.ka))
        ka_input.textEdited.connect(
            lambda: self.set_variable("ka", ka_input.text()))
        grid.addWidget(ka_label, 11, 2)
        grid.addWidget(ka_input, 11, 3)

        il_label = QLabel("Il")
        il_label.setAlignment(Qt.AlignCenter)
        il_input = QLineEdit()
        il_input.setAlignment(Qt.AlignCenter)
        il_input.setValidator(self.float_validator)
        il_input.setText(str(self.il))
        il_input.textEdited.connect(
            lambda: self.set_variable("il", il_input.text()))
        grid.addWidget(il_label, 12, 2)
        grid.addWidget(il_input, 12, 3)

        n_label = QLabel("N")
        n_label.setAlignment(Qt.AlignCenter)
        n_input = QLineEdit()
        n_input.setAlignment(Qt.AlignCenter)
        n_input.setValidator(self.float_validator)
        n_input.setText(str(self.n))
        n_input.textEdited.connect(
            lambda: self.set_variable("n", n_input.text()))
        grid.addWidget(n_label, 13, 2)
        grid.addWidget(n_input, 13, 3)

        # Reset button
        reset_button = QPushButton("Reset")
        reset_button.clicked.connect(lambda: self.reset())
        # Expand button to fill the whole column
        grid.addWidget(reset_button, 6, 0)

        # Apply button
        apply_button = QPushButton("Apply")
        apply_button.clicked.connect(lambda: self.apply())
        # Expand button to fill the whole column

        grid.addWidget(apply_button, 6, 1)

        grid.setColumnStretch(0, 1)
        grid.setRowStretch(0, 1)

        # add padding
        grid.setContentsMargins(20, 20, 20, 20)

        display_grid_on_window(window, grid)
