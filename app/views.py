from PySide2 import Qt, QtCore, QtWidgets, QtGui

import matplotlib.pyplot as plt
import matplotlib.patches as patches

from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
from matplotlib.backends.backend_qt5agg import NavigationToolbar2QT as NavigationToolbar
from matplotlib.figure import Figure

from analysis import KeyPress

import numpy as np

plt.rcParams['axes.grid'] = True

class FilePicker(QtWidgets.QWidget):

    def __init__(self):
        super(FilePicker, self).__init__()

        self.layout = QtWidgets.QHBoxLayout(self)

        self.dir = QtWidgets.QLineEdit()
        self.btn = QtWidgets.QPushButton('BROWSE')

        self.btn.clicked.connect(
            lambda x: self.dir.setText(
                QtWidgets.QFileDialog.getExistingDirectory()
            )
        )

        self.layout.addWidget(self.dir)
        self.layout.addWidget(self.btn)

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

        self.setStyleSheet("font-weight: bold; font-size: {}px".format(24))
        self.forceResult = QtWidgets.QLabel('3.5 N')
        self.accelResult = QtWidgets.QLabel('20 mm/s^2')
        self.encoder = QtWidgets.QLabel('-')
        self.risetimeResult = QtWidgets.QLabel('- s')


        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Encoder'))
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
        self.clear()
        t, speed, speed_fitted = k.speed_data()
        self.plot(t, speed, speed_fitted, title='Speed vs time', xlabel='Time [ms]', ylabel='Speed [mm/s]')


    def show_accel(self, k: KeyPress):
        self.clear()
        t, accel, accel_fitted = k.accel_data()
        self.plot(t, accel, accel_fitted, title='Acceleration vs time', xlabel='Time [ms]', ylabel='Speed [mm/s^2]', plot_z_average=True)


    def show_position(self, k: KeyPress):
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


        self.output = QtWidgets.QTextEdit()
        self.output.setReadOnly(True)
        self.input = QtWidgets.QLineEdit()

        self.layout = QtWidgets.QVBoxLayout(self)

        self.layout.addWidget(self.output)

        self.inputLayout = QtWidgets.QHBoxLayout()
        self.inputLayout.addWidget(self.input)

        self.submitBtn = QtWidgets.QPushButton("Enter")
        self.inputLayout.addWidget(self.submitBtn)

        self.layout.addWidget(self.input)

        self.layout.addWidget(QtWidgets.QLabel('Add comments or markers to log output'))
        self.layout.addLayout(self.inputLayout)


    @QtCore.Slot(str)
    def addText(self, text: str):
        self.output.append(text)


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

    @QtCore.Slot(list)
    def updateCOMPorts(self, portlist):
        self.dropdown.clear()
        self.dropdown.addItems(portlist)
