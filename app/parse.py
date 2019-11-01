from PySide2 import Qt, QtCore

START = 'Start Encoder'
END = 'End'

class SerialParser(QtCore.QObject):


    newDataSet = QtCore.Signal(list, list)

    def __init__(self):
        super(SerialParser, self).__init__()
        self.started = False
        self.timestamps = []
        self.positions = []

    @QtCore.Slot(str)
    def parse_line(self, line: str):

        if line.startswith(END):
            self.started = False
            self.newDataSet.emit(self.timestamps, self.positions)

        if line.startswith(START):
            encoder = int(line[len(START):])
            self.started = True
            self.timestamps = []
            self.positions = []
            return

        if self.started:
            res = line.split(":")
            self.timestamps.append(int(res[0]))
            self.positions.append(-1*int(res[1]))





