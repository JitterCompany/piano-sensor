import random
from PySide2 import Qt, QtCore, QtWidgets, QtGui
import serial
from serial.tools import list_ports

import matplotlib.pyplot as plt
from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
from matplotlib.backends.backend_qt5agg import NavigationToolbar2QT as NavigationToolbar
from matplotlib.figure import Figure


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

        self.layout = QtWidgets.QVBoxLayout(self)
        self.plot = MatplotlibWidget()
        self.layout.addWidget(self.plot)
        self.plot.update_plot(range(5))

        self.forceResult = QtWidgets.QLabel('3.5 N')
        self.accelResult = QtWidgets.QLabel('20 mm/s^2')

        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Force'))
        valueLayout.addWidget(self.forceResult)

        self.layout.addLayout(valueLayout)

        valueLayout = QtWidgets.QHBoxLayout()
        valueLayout.addWidget(QtWidgets.QLabel('Acceleration'))
        valueLayout.addWidget(self.accelResult)

        self.layout.addLayout(valueLayout)


class TextOutputView(QtWidgets.QWidget):

      def __init__(self, parent=None):
        super(TextOutputView, self).__init__()


        self.output = QtWidgets.QTextEdit()
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
        self.line, *_ = self.ax.plot([])

    # @Slot(list)
    def update_plot(self, data):
        self.line.set_data(range(len(data)), data)

        self.ax.set_xlim(0, len(data))
        self.ax.set_ylim(min(data), max(data))
        self.canvas.draw()

class MainView(QtWidgets.QWidget):
    def __init__(self, toolbar):
        super(MainView, self).__init__()

        availablePorts = list_ports.comports()

        portsString = ''.join([p.device + '\n' for p in availablePorts])

        self.hello = ["Hallo Welt", "Hei maailma", "Hola Mundo", "Привет мир"]


        self.toolbar = toolbar


        self.button = QtWidgets.QPushButton("Click me!")
        self.text = QtWidgets.QLabel("Text")
        self.text.setAlignment(QtCore.Qt.AlignCenter)



        self.dropdown = QtWidgets.QComboBox()
        self.dropdown.addItems([p.device for p in availablePorts])
        self.toolbar.addWidget(QtWidgets.QLabel("Select COM Port:"))
        self.toolbar.addWidget(self.dropdown)

        self.filepicker = FilePicker()

        empty =  QtWidgets.QWidget()

        empty.setSizePolicy(QtWidgets.QSizePolicy.Expanding, QtWidgets.QSizePolicy.Preferred)
        self.toolbar.addSeparator()
        self.toolbar.addWidget(empty)

        self.toolbar.addWidget(QtWidgets.QLabel("Log directory:"))

        self.toolbar.addWidget(self.filepicker)

        self.layout = QtWidgets.QHBoxLayout(self)

        self.left = ResultView()
        self.right = TextOutputView()


        self.layout.addWidget(self.left)
        self.layout.addWidget(self.right)

        self.button.clicked.connect(self.magic)




    def magic(self):
        self.text.setText(random.choice(self.hello))