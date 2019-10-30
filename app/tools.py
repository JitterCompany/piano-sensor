""" Some utility functions for PyQt widgets """

from PySide2.QtGui import QPalette, QColor
from PySide2 import QtWidgets

def set_background_color(widget, color: str):
    """ set widget color to color: str"""
    pal = QPalette()
    # set black background
    pal.setColor(QPalette.Background, QColor(color))
    widget.setAutoFillBackground(True)
    widget.setPalette(pal)

def set_button_color(button, color: str):
    """ set button color to color: str"""
    pal = button.palette()
    pal.setColor(QPalette.Button, QColor(color))
    button.setAutoFillBackground(True)
    button.setPalette(pal)
    button.update()

def bold_label(text, size=13):
    """ Create a label with font-weight bold """
    label = QtWidgets.QLabel(text)
    label.setStyleSheet("font-weight: bold; font-size: {}px".format(size))
    label.update()
    return label

def spacer():
    """ Returns empty widget that expands to all available space """
    s = QtWidgets.QWidget()
    s.setSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred)
    return s



class Row(QtWidgets.QWidget):
    """Easily add widgets in a horizontal layout"""

    def __init__(self, *widgets):
        super().__init__()
        layout = QtWidgets.QHBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        for w in widgets:
            layout.addWidget(w)

class Column(QtWidgets.QWidget):
    """Easily add widgets in a vertical layout"""

    def __init__(self, *widgets):
        super().__init__()
        layout = QtWidgets.QVBoxLayout(self)
        layout.setContentsMargins(0, 0, 0, 0)
        for w in widgets:
            layout.addWidget(w)
