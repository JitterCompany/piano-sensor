import sys
import random
import signal
from PySide2 import Qt, QtCore, QtWidgets, QtGui

import serial
from serial.tools import list_ports

from tools import set_background_color

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

class MyWidget(QtWidgets.QWidget):
    def __init__(self):
        super(MyWidget, self).__init__()

        availablePorts = list_ports.comports()

        portsString = ''.join([p.device + '\n' for p in availablePorts])

        self.hello = ["Hallo Welt", "Hei maailma", "Hola Mundo", "Привет мир"]

        self.button = QtWidgets.QPushButton("Click me!")
        self.text = QtWidgets.QLabel(portsString)
        self.text.setAlignment(QtCore.Qt.AlignCenter)

        self.layout = QtWidgets.QVBoxLayout()
        self.layout.addWidget(self.text)
        self.layout.addWidget(self.button)
        self.setLayout(self.layout)

        self.button.clicked.connect(self.magic)




    def magic(self):
        self.text.setText(random.choice(self.hello))


if __name__ == "__main__":
    app = QtWidgets.QApplication([])

    widget = MyWidget()

    window = MainWindow()
    window.setCentralWidget(widget)

    window.show()
    signal.signal(signal.SIGINT, window.quit)
    # signal.signal(signal.SIGINT, signal.SIG_DFL)

    if sys.flags.interactive != 1:
        sys.exit(app.exec_())
