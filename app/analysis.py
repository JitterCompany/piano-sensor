
import numpy as np
import pandas as pd

from PySide2 import QtCore

from settings import DOWNWEIGHTS_g

TICKS_PER_MM = 75/25.4 * 4

class KeyPress:

    def __init__(self, timestamps: list, positionData: list):

        record_threshold_min_mm = 1
        complete_threshold_mm = 15
        self.timestamps, i = np.unique(np.array(timestamps), return_index=True)
        self.positionData = np.array(positionData)[i] / TICKS_PER_MM

        # Find index for T_0 where key starts to go down significantly
        first_index = np.argmax(self.positionData > record_threshold_min_mm)

        # Index for T_1, key push (almost) complete
        last_index = np.argmax(self.positionData > complete_threshold_mm)

        if last_index:
            self.y = self.positionData[first_index:last_index]
            self.t = self.timestamps[first_index:last_index]
        else:
            self.y = None
            self.t = None

    def dt(self):
        return self.t[-1] - self.t[0]

    def dy(self):
        return self.y[-1] - self.y[0]

    def valid(self):
        return self.y is not None

    def metrics(self):

        t, accel, accel_polyfit = self.accel_data()
        average_acceleration = np.mean(accel_polyfit)

        rise_time = self.t[-1] - self.t[0]

        return rise_time, average_acceleration

    def speed_data(self):
        time_s = self.t / 1000
        speed = np.gradient(self.y, time_s)

        MODEL_ORDER = 2
        coeffs = np.polyfit(self.t, speed, MODEL_ORDER)

        poly = np.poly1d(coeffs)
        speed_polyfit = [poly(x) for x in self.t]

        return (self.t, speed, speed_polyfit)

    def accel_data(self):

        t, speed, speed_polyfit = self.speed_data()

        time_s = self.t / 1000

        accel = np.gradient(speed, time_s)
        accel_polyfit = np.gradient(speed_polyfit, time_s)

        return self.t, accel, accel_polyfit


    def constantAccel(self):

        press_time_sec = (self.t[-1]  - self.t[0]) / 1000

        const_accel = 2*self.y[-1]/press_time_sec**2

        return const_accel

    def averageAccel(self):

        time_s = self.t / 1000
        speed = np.gradient(self.y, time_s)
        accel = np.gradient(speed, time_s)

        return np.average(accel)

    # def sliceRectangle(self, record_threshold_min_mm = 1, complete_threshold_mm = 10)


class Analysis(QtCore.QObject):

    textStream = QtCore.Signal(str)

    def __init__(self):
        super(Analysis, self).__init__()


    def estimateAcceleration(self, timestamps: list, positionData: list):
        """

        record_threshold_min_mm: int
            minimum distance after which we start measuring
        complete_threshold_mm: int
            distance the key needs to travel in order to count it as a real push
        """

        keyPress = KeyPress(timestamps, positionData)

        # timestamps = np.array(timestamps)
        # positionData = np.array(positionData) / TICKS_PER_MM

        # # Find index for T_0 where key starts to go down significantly
        # first_index = np.argmax(positionData > record_threshold_min_mm)

        # # Index for T_1, key push (almost) complete
        # last_index = np.argmax(positionData > complete_threshold_mm)

        # if not last_index:
        #     raise Exception("Threshold distance not reached!")

        # dataslice = positionData[first_index:last_index]
        # timeslice = timestamps[first_index:last_index]

        # width = timeslice[-1] - timeslice[0]
        # height = dataslice[-1] - dataslice[0]


        # press_time_sec = (timestamps[last_index]  - timestamps[first_index]) / 1000

        # # Acceleration (assuming it is constant) in mm/s**2
        # const_accel = 2*dataslice[-1]/press_time_sec**2
        # print("Estimated accel: {:.0f} mm/s² ({:.1f} mm in {:.3} sec)".format(const_accel, dataslice[-1], press_time_sec))

        # speed = np.gradient(dataslice, timeslice/1000)
        # accel = np.gradient(speed, timeslice/1000)

        # accel_check = (speed[-1] - speed [0]) / press_time_sec

        # MODEL_ORDER = 2
        # coeffs = np.polyfit(timeslice, speed, MODEL_ORDER)

        # poly = np.poly1d(coeffs)
        # speed_polyfit = [poly(x) for x in timeslice]
        # accel_polyfit = np.gradient(speed_polyfit, timeslice/1000)


        # average_accel = np.average(accel)
        # average_accel_ployfit = np.average(accel_polyfit)


        # index = pd.to_timedelta(timeslice, unit='ms')
        # df = pd.DataFrame(index=index, data=accel)
        # ds = df.resample('5ms').mean()

        # ds_avg = ds.mean()[0]

        # print('Average acceleration: ', average_accel)
        # # Create a Rectangle patch
        # # rect = patches.Rectangle((timeslice[0],dataslice[0]),width,height,linewidth=1,
        # #                          edgecolor='r',facecolor='none', linestyle='--')

