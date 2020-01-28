# Piano Sensor Project

- Electronic design
- Firmware written in Rust for stm32f103
- Python app for data visualization


## Electronic Design

This repo contains the following KiCad projects

* encoder-breakout is the breakout pcb for the AEDR-8300 optical encoder IC
* main-board is a daisychainable board that connects to five encoder-breakouts
* uart-adapter connects to the main-board and delivers power. 


## Firmware

There are two firmware projects

* `firmware` was the first try. It works and decodes the encoder signals but does not have all functionality such as daisychaining.
* `firmware-rftm` is firmware based on the [`cortex-m-rtfm`](https://rtfm.rs) framework. This has all functionality and is compatible with the python app.


## App

The python app lives in a Pipenv virtual environment. First time you need to run the following command to install:

```bash
pipenv install
```

Then to run:

```
pipenv shell
python app.py
```
 
