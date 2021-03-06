import sys
import random
import signal
from PySide2 import Qt, QtCore, QtWidgets, QtGui

import serial
from serial.tools import list_ports

from tools import set_background_color
from views import MainView

from comports import SerialConnection, SerialParser
from analysis import KeyPress

import status

FPS = 20


class PianoApp(QtWidgets.QApplication):

    def __init__(self):
        super(PianoApp, self).__init__()

        self.SerialConnection = SerialConnection()


        self.window = MainWindow()
        self.window.setWindowTitle("Piano Sensor")

        self.window.show()
        signal.signal(signal.SIGINT, self.window.quit)
        self.window.closeSignal.connect(self.quit)

        self.toolbar = QtWidgets.QToolBar()
        self.window.addToolBar(self.toolbar)
        self.mainView = MainView(self.toolbar, self.SerialConnection.getDropdownWidget())
        self.window.setCentralWidget(self.mainView)

        self.mainView.refresh.connect(self.SerialConnection.refresh)

        self.mainView.resetEncoders.connect(lambda: self.SerialConnection.sendCmd('reset'))
        self.mainView.resetSystem.connect(lambda: self.SerialConnection.sendCmd('sysreset'))
        self.mainView.getPositions.connect(lambda: self.SerialConnection.sendCmd('pos'))


        self.parser = SerialParser()
        self.SerialConnection.textStream.connect(self.parser.parse_line)
        # self.SerialConnection.textStream.connect(self.mainView.textOutputView.addText)

        self.parser.comment.connect(self.mainView.textOutputView.addComment)

        self.parser.newDataSet.connect(lambda i, t, p: self.mainView.resultsView.new_results(KeyPress(i, t,p)))
        self.parser.newDataSet.connect(lambda i, t, p: self.mainView.textOutputView.new_results(KeyPress(i, t,p)))

        status.set_status_logger(self.set_status_message)
        status.set_status('Piano Sensor Ready..')

    def quit(self):
        self.mainView.textOutputView.quit()

    def set_status_message(self, msg: str):
        self.window.statusBar().showMessage(msg)


class MainWindow(QtWidgets.QMainWindow):
    """
    Class docstring
    """

    closeSignal = QtCore.Signal()


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
        self.setWindowIcon(QtGui.QIcon('assets/icon.jpeg'))
        self.setGeometry(50, 50, 1600, 900)
        # set_background_color(self, '#5a5d73')
        # set_background_color(self, 'gray')

        self.statusBar().showMessage('')

        self._center()
        self.raise_()
        self.activateWindow()


    def closeEvent(self, event):
        """Handle window close event"""
        if event:
            self.closeSignal.emit()
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

        # qRect = self.frameGeometry()
        # centerPoint = QtWidgets.QDesktopWidget().availableGeometry().center()
        # qRect.moveCenter(centerPoint)
        # self.move(qRect.topLeft())



    def _update(self):
        """ Gui Thread poll """
        if not self._running:
            # not running, pass update
            self.close()
            return
        pass

if __name__ == "__main__":
    app = PianoApp()

    if sys.flags.interactive != 1:
        sys.exit(app.exec_())
