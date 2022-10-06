from PyQt5.QtWidgets import QMainWindow
from gui.color_converter import ColorConverter
import gui.qt_override as qto
from gui import fs
from modules.operations import CG


class MenuAction:
    def __init__(self, name, function, shortcut=None, tooltip=None):
        self.name = name
        self.function = function
        self.shortcut = shortcut
        self.tooltip = tooltip

    def get_values(self):
        return self.name, self.function, self.shortcut, self.tooltip

    def get_function(self):
        return self.function

    def get_name(self):
        return self.name

    def get_shortcut(self):
        return self.shortcut

    def get_tooltip(self):
        return self.tooltip


def display_menubar(window: QMainWindow):
    menubar = window.menuBar()
    window.setMenuBar(menubar)
    add_menus_to_menubar(window, menubar)


def add_menus_to_menubar(window, menubar):
    menus = (
        MenuAction("File", lambda: add_actions_to_file_menu(window)),
        MenuAction("Tools", lambda: add_actions_to_tools_menu(window)),
    )
    for menu in menus:
        new_menu = menubar.addMenu(menu.get_name())
        add_submenus_to = menu.get_function()
        add_submenus_to(new_menu)


def add_actions_to_menu(window, menu, actions):
    for action in actions:
        name, func, shortcut, tooltip = action.get_values()
        act = qto.add_submenu(window, name, func, shortcut, tooltip)
        menu.addAction(act)


def add_actions_to_file_menu(window, file_menu):
    actions = (
        MenuAction("Open", fs.open_image, "CTRL+O", "Open an image"),
        MenuAction("Save", fs.save_image, "CTRL+S", "Save the image"),
        MenuAction("Exit", window.close, "CTRL+Q", "Exit the application"),
    )
    add_actions_to_menu(file_menu, actions)


def add_actions_to_tools_menu(window, tools_menu):
    actions = (
        MenuAction("Color Converter", lambda: ColorConverter(window)),
    )
    add_actions_to_menu(tools_menu, actions)
