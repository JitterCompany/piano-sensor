
from PySide2 import QtCore, QtWidgets

import threading
import codecs
import serial
from serial.tools import list_ports

from serial.tools import hexlify_codec

codecs.register(lambda c: hexlify_codec.getregentry() if c == 'hexlify' else None)

class SerialManager(QtCore.QObject):


    # newCOMPorts = QtCore.Signal(list)

    textStream = QtCore.Signal(str)

    def __init__(self):
        super(SerialManager, self).__init__()

        self.dropdown = QtWidgets.QComboBox()
        self.dropdown.currentIndexChanged.connect(self.selectCOMPort)

        self.availablePorts = None
        self.refresh()

        self.serial = None
        self.alive = None
        self._reader_alive = None
        self.receiver_thread = None

        self.rx_decoder = codecs.getincrementaldecoder('UTF-8')('replace')


        # portsString = ''.join([p.device + '\n' for p in self.availablePorts])

    @QtCore.Slot(str)
    def selectCOMPort(self, index):
        if index:
            port = self.availablePorts[index-1]
            print('select', port)
            self.change_port(port)
        else:
            self._stop_reader()
            self.serial = None


    def getDropdownWidget(self):
        return self.dropdown

    def refresh(self):
        self.availablePorts = list_ports.comports()
        self.dropdown.clear()
        self.dropdown.addItem('--- Select COM Port ---')
        self.dropdown.addItems([p.device for p in self.availablePorts])
        # self.newCOMPorts.emit([p.device for p in self.availablePorts])



    def change_port(self, port: serial.Serial):

        if port and self.serial and port != self.serial.port:
            # reader thread needs to be shut down
            self._stop_reader()
        self.serial = serial.Serial(port.device, 115200, timeout=1)
        print('open port: ', self.serial, self.serial.port)
        self._start_reader()


    def _start_reader(self):
        """Start reader thread"""
        print('Start reader thread')
        self._reader_alive = True
        # start serial->console thread
        self.receiver_thread = threading.Thread(target=self.reader, name='rx')
        self.receiver_thread.daemon = True
        self.receiver_thread.start()

    def _stop_reader(self):
        """Stop reader thread only, wait for clean exit of thread"""
        self._reader_alive = False
        if hasattr(self.serial, 'cancel_read'):
            self.serial.cancel_read()
        self.receiver_thread.join()

    def reader(self):
        """loop and copy serial->console"""
        try:
            while self._reader_alive:
                # read all that is there or wait for one byte
                data = self.serial.read(self.serial.in_waiting or 1)
                if data:
                    text = self.rx_decoder.decode(data)
                    # for transformation in self.rx_transformations:
                    #     text = transformation.rx(text)
                    print(text)
                    self.textStream.emit(text)
                    # self.console.write(text)

        except serial.SerialException:
            self.alive = False
            # self.console.cancel()
            raise       # XXX handle instead of re-raise?
