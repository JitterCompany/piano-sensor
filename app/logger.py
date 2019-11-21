"""
Logger is a functional module for writing csv files

A new logging session is started with `logger.start_new_session()`.
With the returned session handle data can be written with `logger.start()`.
The session can be ended with `logger.close_session()`. The file will then be closed.
"""

import csv
import typing
from pathlib import Path

LogHandle = typing.NamedTuple('loghandle', [('file', typing.TextIO), ('writer', csv.writer)])

def start_new_session(directory, file_prefix: str, csv: bool):
    """
    Parameters
    ----------
    directory: str
        The directory files will be logged to. If it does not exists,
        logger tries to create the directory

    file_prefix: str
        The prefix in the file name.

        e.g. for 'somefile' the filename will be 'somefile_i.csv'
        with i the sequence number of the logging session.

        The logger will check what the latest session with 'file_prefix' in
        'directory' was and choose the next value for 'i'

    Returns
    -------
    logging_handle: LogHandle
        use this handle for writing data during the session

    None if there was a problem
    """

    ext = 'csv' if csv else 'txt'
    p = Path(directory)
    if not p.exists():
        try:
            p.mkdir(parents=True)
        except Exception as e:
            print('error creating log dir: \n', e)
            return None

    files = list(p.glob(file_prefix + '*' + ext))
    max_i = -1
    for f in files:
        try:
            i = int(f.stem[len(file_prefix+'_'):])
            max_i = max(i, max_i)
        except ValueError as e:
            print(e)
    try:
        fname = p / '{}_{}.{}'.format(file_prefix, max_i+1, ext)
        csvfile = open(str(fname), 'w', newline='')
        writer = csv.writer(csvfile, delimiter=',') if csv else None
        print('starting new logging session with file:', str(fname))
        return LogHandle(file=csvfile, writer=writer)
    except Exception as e:
        print('error opening file: \n', e)

    return None

def write_str(handle: LogHandle, line: str):
    """
    Write a string to the file

    Parameters
    ----------
    handle: LogHandle
        The handle returned by 'start_new_session'
    line: str
    """
    if handle and handle.file and line is not None:
        print('write to file:', line)
        handle.file.write(line)

def write_csv(handle: LogHandle, data: typing.Iterable):
    """
    Write csv data to file

    Parameters
    ----------
    handle: LogHandle
        The handle returned by 'start_new_session'
    data: Iterable
        Iterable for which each item represents a line in the csv file.
        Each line needs an array or list of data

        For example a 2d numpy array:

            np.array([[1, 2, 3], [4, 5, 6])

            results in csv file contents:
            1,2,3
            4,5,6
    """
    if handle and handle.writer and data is not None:
        handle.writer.writerows(data)

def close_session(handle: LogHandle):
    """
    Closes session and csvfile on disk
    """
    if handle and handle.file:
        print('closing logging session for file:', handle.file.name)
        handle.file.close()
