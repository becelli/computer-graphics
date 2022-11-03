import sys
from PyQt5.QtWidgets import QApplication
from gui.index import Application

if __name__ == "__main__":
    app = QApplication(sys.argv)
    window = Application()
    window.show()
    sys.exit(app.exec())
