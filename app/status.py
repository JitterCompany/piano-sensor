

LOGGER = None

def set_status_logger(logger):
    global LOGGER
    LOGGER = logger


def set_status(msg):
    if LOGGER:
        LOGGER(msg)