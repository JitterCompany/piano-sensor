import sys
import random
import signal
from PySide2 import Qt, QtCore, QtWidgets, QtGui

import serial
from serial.tools import list_ports

from tools import set_background_color
from mainview import MainView

FPS = 20

class MainWindow(QtWidgets.QMainWindow):
    """
    Class docstring
    """

    def __init__(self):
        super(MainWindow, self).__init__()

        self._running = True
        exitShortcut = QtWidgets.QShortcut("CTRL+Q", self)
        exitShortcut.activated.connect(self.close) # calls closeEvent

        self._setupView()

        self.timer = QtCore.QTimer()
        self.timer.timeout.connect(self._update)
        self.timer.start(1000/FPS)

        self.popup = None

    def _setupView(self):
        """Initialize Main Window"""
        # self.setWindowIcon(QtGui.QIcon('assets/icon.png'))
        self.setGeometry(50, 50, 1600, 900)
        self.setWindowTitle("Serial Console")
        set_background_color(self, 'white')

        # self.setCentralWidget(self.terminal.view)
        # self._center()
        # self.raise_()
        # self.activateWindow()

    def closeEvent(self, event):
        """Handle window close event"""
        if event:
            # self.terminal.close()
            event.accept()
            if not self._running:
                return
            print('shutting down..')
            self._running = False

    def quit(self, _signal=None, _=None):
        """ Signal Handler to quit the program """
        self.close()

    def _error_popup(self, msg):
        QtWidgets.QMessageBox.critical(self, "Unexpected Error", msg)

    def _center(self):
        """ Center Window on current display """
        frameGm = self.frameGeometry()
        screen = QtWidgets.QApplication.desktop().screenNumber(QtWidgets.QApplication.desktop().cursor().pos())
        centerPoint = QtWidgets.QApplication.desktop().screenGeometry(screen).center()
        frameGm.moveCenter(centerPoint)
        self.move(frameGm.topLeft())

    def _update(self):
        """ Gui Thread poll """
        if not self._running:
            # not running, pass update
            self.close()
            return
        pass

if __name__ == "__main__":
    app = QtWidgets.QApplication([])


    window = MainWindow()


    window.show()
    signal.signal(signal.SIGINT, window.quit)
    # signal.signal(signal.SIGINT, signal.SIG_DFL)

    toolbar = QtWidgets.QToolBar()
    window.addToolBar(toolbar)
    widget = MainView(toolbar)
    window.setCentralWidget(widget)

    if sys.flags.interactive != 1:
        sys.exit(app.exec_())
