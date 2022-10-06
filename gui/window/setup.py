from PyQt5.QtGui import QGuiApplication


def set_window_properties(window) -> None:
    window.setWindowTitle("Computer Graphics")
    center_window(window)


def center_window(window) -> None:
    temporary_window = window.frameGeometry()
    center_point = QGuiApplication.primaryScreen().availableGeometry().center()
    temporary_window.moveCenter(center_point)
    window.move(temporary_window.topLeft())
