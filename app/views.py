from PySide2 import Qt, QtCore, QtWidgets, QtGui

import matplotlib.pyplot as plt
import matplotlib.patches as patches

from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
from matplotlib.backends.backend_qt5agg import NavigationToolbar2QT as NavigationToolbar
from matplotlib.figure import Figure

from analysis import KeyPress

import numpy as np

from settings import LOG_DIR, FILE_PREFIX
import logger

plt.rcParams['axes.grid'] = True

class FilePicker(QtWidgets.QWidget):

    updateDir = QtCore.Signal(str)

    def __init__(self):
        super(FilePicker, self).__init__()

        self.layout = QtWidgets.QHBoxLayout(self)

        self.dir = QtWidgets.QLineEdit(LOG_DIR)
        self.btn = QtWidgets.QPushButton('Browse')
        self.newBtn = QtWidgets.QPushButton('New Session')
        self.newBtn.clicked.connect(lambda x: self._pick(self.dir.text()))

        self.btn.clicked.connect(
            lambda x: self._pick(
                QtWidgets.QFileDialog.getExistingDirectory()
            )
        )
        self.updateDir.emit(self.dir.text())

        self.layout.addWidget(self.dir)
        self.layout.addWidget(self.btn)
        self.layout.addWidget(self.newBtn)

    def _pick(self, directory: str):
        self.dir.setText(directory)
        self.updateDir.emit(directory)



class ResultView(QtWidgets.QWidget):

    def __init__(self):
        super(ResultView, self).__init__()

        self.current_keypress = None

        self.layout = QtWidgets.QVBoxLayout(self)
        self.plot = MatplotlibWidget()
        self.layout.addWidget(self.plot)


        buttons = QtWidgets.QHBoxLayout()
        btn = QtWidgets.QPushButton('position')
        btn.clicked.connect(lambda: self.plot.show_position(self.current_keypress))
        buttons.addWidget(btn)
        btn = QtWidgets.QPushButton('speed')
        btn.clicked.connect(lambda: self.plot.show_speed(self.current_keypress))
        buttons.addWidget(btn)
        btn = QtWidgets.QPushButton('accel')
        btn.clicked.connect(lambda: self.plot.show_accel(self.current_keypress))
        buttons.addWidget(btn)

        self.layout.addLayout(buttons)

        # self.plot.update_plot(range(5))

        # self.setStyleSheet("font-weight: bold; font-size: {}px".format(24))
        self.forceResult = QtWidgets.QLabel('- N')
        self.accelResult = QtWidgets.QLabel('- mm/s^2')
        self.encoder = QtWidgets.QLabel('-')
        self.risetimeResult = QtWidgets.QLabel('- s')


        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Key'))
        valueLayout.addWidget(self.encoder)
        self.layout.addLayout(valueLayout)

        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Force'))
        valueLayout.addWidget(self.forceResult)

        self.layout.addLayout(valueLayout)

        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Acceleration'))
        valueLayout.addWidget(self.accelResult)

        self.layout.addLayout(valueLayout)

        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Rise Time'))
        valueLayout.addWidget(self.risetimeResult)

        self.layout.addLayout(valueLayout)



    @QtCore.Slot(KeyPress)
    def new_results(self, k: KeyPress):
        if k.valid():
            self.encoder.setText(str(k.encoder))
            self.current_keypress = k
            rise_time, avg_accel, force = self.current_keypress.metrics()
            self.plot.show_position(self.current_keypress)
            self.accelResult.setText('{0:.2f} mm/s^2'.format(avg_accel))
            self.risetimeResult.setText('{0:.1f} ms'.format(rise_time))
            self.forceResult.setText('{0:.2f} N'.format(force))


class MatplotlibWidget(QtWidgets.QWidget):
    def __init__(self, parent=None):
        super().__init__(parent)

        fig = Figure(figsize=(7, 5), dpi=65, facecolor=(1, 1, 1), edgecolor=(0, 0, 0))
        self.canvas = FigureCanvas(fig)
        self.toolbar = NavigationToolbar(self.canvas, self)
        lay = QtWidgets.QVBoxLayout(self)
        lay.addWidget(self.toolbar)
        lay.addWidget(self.canvas)

        self.ax = fig.add_subplot(111)
        self.ax.set_xlabel('Time [ms]')
        self.ax.set_ylabel('Position [mm]')
        self.ax.set_title('Keypress position vs time')

        self.line1, *_ = self.ax.plot([])
        self.line2, *_ = self.ax.plot([])
        self.rect = None


    def plot(self, x, y, z, title, xlabel, ylabel, plot_z_average=False):
        self.clear()
        self.ax.set_xlabel(xlabel)
        self.ax.set_ylabel(ylabel)
        self.ax.set_title(title)

        self.ax.plot(x, y, label='raw')

        self.ax.set_xlim(min(x), max(x))
        self.ax.set_ylim(min(y), max(y))

        if z is not None and len(z):
            self.ax.plot(x, z, label='fitted')
            if plot_z_average:
                self.ax.plot(x, np.repeat(np.mean(z), len(x)), label='mean')

        self.ax.legend()
        self.canvas.draw()

    def clear(self):
        if self.rect:
            self.rect.remove()
            self.rect = None
        self.ax.clear()

    def show_speed(self, k: KeyPress):

        if not k:
            return

        self.clear()
        t, speed, speed_fitted = k.speed_data()
        self.plot(t, speed, speed_fitted, title='Speed vs time', xlabel='Time [ms]', ylabel='Speed [mm/s]')


    def show_accel(self, k: KeyPress):

        if not k:
            return

        self.clear()
        t, accel, accel_fitted = k.accel_data()
        self.plot(t, accel, accel_fitted, title='Acceleration vs time', xlabel='Time [ms]', ylabel='Speed [mm/s^2]', plot_z_average=True)


    def show_position(self, k: KeyPress):

        if not k:
            return

        self.clear()

        t = k.timestamps
        y = k.positionData

        self.plot(t, y, [], title='Position vs time', xlabel='Time [ms]', ylabel='Position [mm]')

        # Create a Rectangle patch
        self.rect = patches.Rectangle((k.t[0],k.y[0]),k.dt(),k.dy(),linewidth=1,
                                edgecolor='r',facecolor='none', linestyle='--')

        # Add the patch to the Axes
        self.ax.add_patch(self.rect)
        self.canvas.draw()


class TextOutputView(QtWidgets.QWidget):

    def __init__(self, parent=None):
        super(TextOutputView, self).__init__()


        self.textView = QtWidgets.QTextEdit()
        self.textView.setReadOnly(True)
        self.textView.setFont(QtGui.QFont ("Courier", 15))
        self.input = QtWidgets.QLineEdit()

        self.layout = QtWidgets.QVBoxLayout(self)

        headerLayout = QtWidgets.QHBoxLayout()
        headerLayout.addWidget(QtWidgets.QLabel("Raw Text Output"))
        headerLayout.addStretch(1)

        self.clearBtn = QtWidgets.QPushButton("Clear")
        self.clearBtn.clicked.connect(self.clear)
        headerLayout.addWidget(self.clearBtn)

        inputbox = QtWidgets.QGroupBox(
                    title='Add comments or markers to log output')
        self.inputLayout = QtWidgets.QHBoxLayout()
        inputbox.setLayout(self.inputLayout)

        self.submitBtn = QtWidgets.QPushButton("Enter")
        self.submitBtn.clicked.connect(self.addComment)

        self.inputLayout.addWidget(self.input)
        self.inputLayout.addWidget(self.submitBtn)


        # Add everything to main layout
        self.layout.addLayout(headerLayout)
        self.layout.addWidget(self.textView)
        self.layout.addWidget(inputbox)

        self.logHandle = logger.start_new_session(LOG_DIR, FILE_PREFIX, csv=False)


    def quit(self):
        logger.close_session(self.logHandle)

    def new_log_session(self, logdir: str):
        if self.logHandle:
            logger.close_session(self.logHandle)
            self.logHandle = None

        self.logHandle = logger.start_new_session(logdir, FILE_PREFIX, csv=False)

    @QtCore.Slot(KeyPress)
    def new_results(self, k: KeyPress):
        self.addText(k.serialize())

    @QtCore.Slot(str)
    def addText(self, text: str):
        self.textView.append(text)
        logger.write_str(self.logHandle, text)

    @QtCore.Slot()
    def addComment(self):
        self.addText(self.input.text())

    @QtCore.Slot()
    def clear(self):
        self.textView.clear()

class MainView(QtWidgets.QWidget):

    refresh = QtCore.Signal()


    def __init__(self, toolbar, dropdown):
        super(MainView, self).__init__()

        self.toolbar = toolbar

        self.button = QtWidgets.QPushButton("Click me!")
        self.text = QtWidgets.QLabel("Text")
        self.text.setAlignment(QtCore.Qt.AlignCenter)


        self.toolbar.addWidget(QtWidgets.QLabel("Select COM Port:"))
        self.toolbar.addWidget(dropdown)

        self.refreshBtn = QtWidgets.QPushButton('Refresh')
        self.refreshBtn.clicked.connect(self.refresh)
        self.toolbar.addWidget(self.refreshBtn)

        self.filepicker = FilePicker()


        empty =  QtWidgets.QWidget()

        empty.setSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred)
        self.toolbar.addSeparator()
        self.toolbar.addWidget(empty)

        self.toolbar.addWidget(QtWidgets.QLabel("Log directory:"))

        self.toolbar.addWidget(self.filepicker)

        self.layout = QtWidgets.QHBoxLayout(self)

        # left
        self.resultsView = ResultView()

        # right
        self.textOutputView = TextOutputView()

        self.layout.addWidget(self.resultsView)
        self.layout.addWidget(self.textOutputView)

        self.filepicker.updateDir.connect(self.textOutputView.new_log_session)

    @QtCore.Slot(list)
    def updateCOMPorts(self, portlist):
        self.dropdown.clear()
        self.dropdown.addItems(portlist)
