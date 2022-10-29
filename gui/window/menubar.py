from PyQt5.QtWidgets import QMainWindow, QMenuBar
from gui.color_converter import ColorConverter
import gui.qt_override as qto
from gui import fs
from modules.operations import CG


class MenuAction:
    def __init__(self, name, function, shortcut=None, tooltip=None):
        self.name: str = name
        self.function: callable = function
        self.shortcut: str = shortcut
        self.tooltip: str = tooltip

    def get_values(self):
        return self.name, self.function, self.shortcut, self.tooltip


def display_menubar(window: QMainWindow):
    menubar: QMenuBar = window.menuBar()
    window.setMenuBar(menubar)
    add_menus_to_menubar(window, menubar)


def add_menus_to_menubar(window: QMainWindow, menubar):
    menus = (
        MenuAction("File", lambda w, m: add_actions_to_file_menu(w, m)),
        MenuAction("Tools", lambda w, m: add_actions_to_tools_menu(w, m)),
    )
    for menu in menus:
        new_menu = menubar.addMenu(menu.name)
        menu.function(window, new_menu)  # add actions to menu


def add_actions_to_menu(window: QMainWindow, menu: QMenuBar, actions: tuple[MenuAction]):
    for action in actions:
        name, func, shortcut, tooltip = action.get_values()
        act = qto.add_submenu(window, name, func, shortcut, tooltip)
        menu.addAction(act)


def add_actions_to_file_menu(window: QMainWindow, file_menu):
    actions = (
        MenuAction("Open", lambda: fs.open_image(
            window.canvas), "CTRL+O", "Open an image"),
        MenuAction("Save", lambda: fs.save_image(
            window.canvas), "CTRL+S", "Save the image"),
        MenuAction("Exit", window.close, "CTRL+Q", "Exit the application"),
    )
    add_actions_to_menu(window, file_menu, actions)


def add_actions_to_tools_menu(window: QMainWindow, tools_menu):
    actions = (
        MenuAction("Color Converter", lambda: ColorConverter(window)),
    )
    add_actions_to_menu(window, tools_menu, actions)
