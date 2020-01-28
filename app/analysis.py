from datetime import datetime

import numpy as np
import pandas as pd

from PySide2 import QtCore

TICKS_PER_MM = 75/25.4 * 4

record_threshold_min_mm = 3
complete_threshold_mm = 13

class KeyPress:

    def __init__(self, encoder: int, timestamps: list, positionData: list):

        self.timestamp = datetime.now()
        self.encoder = encoder

        self.timestamps, i = np.unique(np.array(timestamps), return_index=True)
        self.positionData = np.array(np.abs(positionData))[i] / TICKS_PER_MM


        # Find index for T_0 where key starts to go down significantly
        first_index = np.argmax(self.positionData > record_threshold_min_mm)

        # Index for T_1, key push (almost) complete
        last_index = np.argmax(self.positionData > complete_threshold_mm)

        # print('first_index', first_index)
        # print('last_index', last_index)

        if last_index:
            self.y = self.positionData[first_index:last_index]
            self.t = self.timestamps[first_index:last_index]
        else:
            self.y = None
            self.t = None


    def summary(self):
        string = "\nKey {} @ {}\n".format(self.encoder, self.timestamp.strftime("%-d/%-m/%Y %-H:%M:%S"))
        string += '{0:.2f} mm/s^2\n'.format(self.average_fitted_acceleration())
        string += '{0:.2f} ms\n'.format(self.metrics()[0])
        return string;

    def serialize(self):
        string = "Key {} @ {}\n".format(self.encoder, self.timestamp.strftime("%-d/%-m/%Y %-H:%M:%S"))
        string += "Time[ms] : Pos.[mm]\n"
        for t,p in zip(self.timestamps, self.positionData):
            string += "{t:<9.1F}:{p:< 5.1F}\n".format(t=t, p=p)

        return string


    def dt(self):
        """ Time duration of measurement box """
        return self.t[-1] - self.t[0]

    def dy(self):
        """ difference in position between start and end point of measurement box

        depends on threshold settings
        """
        return self.y[-1] - self.y[0]

    def valid(self):
        """
        returns whether key press was valid
        """
        return self.y is not None

    def metrics(self):
        """
        returns rise_time, average_acceleration

        rise_time in milliseconds
        average_acceleration in mm/s^2 based no fitted speed
        """

        t, accel, accel_polyfit = self.accel_data()
        average_acceleration = np.mean(accel_polyfit)

        rise_time = self.t[-1] - self.t[0]

        return rise_time, average_acceleration

    def average_fitted_acceleration(self):
        t, accel, accel_polyfit = self.accel_data()
        average_acceleration = np.mean(accel_polyfit)
        return average_acceleration

    def speed_data(self):
        """
        returns tuple of
        time [ms] speed and fitted_speed as numpy arrays
        """
        time_s = self.t / 1000
        speed = np.gradient(self.y, time_s)

        MODEL_ORDER = 2
        coeffs = np.polyfit(self.t, speed, MODEL_ORDER)

        poly = np.poly1d(coeffs)
        speed_polyfit = [poly(x) for x in self.t]

        return (self.t, speed, speed_polyfit)

    def accel_data(self):
        """
        returns tuple of
        time [ms] acceleration and fitted_acceleration as numpy arrays
        """

        t, speed, speed_polyfit = self.speed_data()

        time_s = self.t / 1000

        accel = np.gradient(speed, time_s)
        accel_polyfit = np.gradient(speed_polyfit, time_s)

        return self.t, accel, accel_polyfit


    def constantAccel(self):
        """
        returns the constant acceleration based on the measurement box
        """
        press_time_sec = (self.t[-1]  - self.t[0]) / 1000

        const_accel = 2*self.y[-1]/press_time_sec**2

        return const_accel

    def averageAccel(self):
        """
        returns average acceleration based on the measurement box
        """
        time_s = self.t / 1000
        speed = np.gradient(self.y, time_s)
        accel = np.gradient(speed, time_s)

        return np.average(accel)
