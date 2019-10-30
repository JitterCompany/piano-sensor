import random
from PySide2 import Qt, QtCore, QtWidgets, QtGui
import serial
from serial.tools import list_ports

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


        self.layout = QtWidgets.QVBoxLayout()
        self.layout.addWidget(self.text)
        self.layout.addWidget(self.button)
        self.setLayout(self.layout)

        self.button.clicked.connect(self.magic)




    def magic(self):
        self.text.setText(random.choice(self.hello))