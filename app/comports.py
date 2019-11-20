
from PySide2 import QtCore, QtWidgets

import threading
import codecs
import serial
from serial.tools import list_ports

from serial.tools import hexlify_codec

codecs.register(lambda c: hexlify_codec.getregentry() if c == 'hexlify' else None)

BAUDRATE = 115200

class SerialConnection(QtCore.QObject):


    # newCOMPorts = QtCore.Signal(list)

    textStream = QtCore.Signal(str)

    def __init__(self):
        super(SerialConnection, self).__init__()

        self.dropdown = QtWidgets.QComboBox()
        self.dropdown.currentIndexChanged.connect(self.selectCOMPort)

        self.availablePorts = None

        self.serial = None
        self.alive = False
        self._reader_alive = False
        self.receiver_thread = None

        self.refresh()
        # self.rx_decoder = codecs.getincrementaldecoder('UTF-8')('replace')
        self.rx_decoder = codecs.getdecoder('UTF-8') #('replace')


        # portsString = ''.join([p.device + '\n' for p in self.availablePorts])

    @QtCore.Slot(str)
    def selectCOMPort(self, index):
        if index > 0:
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
        self.serial = serial.Serial(port.device, BAUDRATE, timeout=1)
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
        if self.serial and hasattr(self.serial, 'cancel_read'):
            self.serial.cancel_read()

        if self.receiver_thread:
            self.receiver_thread.join()

    def reader(self):
        """loop and copy serial->console"""
        try:
            while self._reader_alive:
                # read all that is there or wait for one byte
                # data = self.serial.read(self.serial.in_waiting or 1)
                data = self.serial.readline()
                if data:
                    # text = self.rx_decoder.decode(data)
                    try:
                        text, length = self.rx_decoder(data[:-1]) # get rid of newline
                        # text = ''.join([chr(c) for c in data])
                        self.textStream.emit(text)
                    except e:
                        print('parse error:', e)
                        pass

        except serial.SerialException:
            self.alive = False
            # self.console.cancel()
            raise       # XXX handle instead of re-raise?


START = 'KEY '
END = 'END'

class SerialParser(QtCore.QObject):


    newDataSet = QtCore.Signal(int, list, list)

    def __init__(self):
        super(SerialParser, self).__init__()
        self.started = False
        self.timestamps = []
        self.positions = []
        self.current_encoder = None

    @QtCore.Slot(str)
    def parse_line(self, line: str):

        if line.startswith(END):
            self.started = False
            self.newDataSet.emit(self.current_encoder, self.timestamps, self.positions)

        if line.startswith(START):
            self.current_encoder = int(line[len(START):])
            self.started = True
            self.timestamps = []
            self.positions = []
            return

        if self.started:
            res = line.split(":")
            self.timestamps.append(int(res[0]) / 10)
            self.positions.append(int(res[1]))


