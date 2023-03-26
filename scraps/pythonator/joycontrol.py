# generated from rust enum

from machine import Pin, SPI
import time
import struct



FRAME_HELLO = 0
FRAME_XY = 1
FRAME_ZT = 2
FRAME_CALLIBRATE = 3
FRAME_DISPLAY = 4
FRAME_FAIL = 5

class controller:
    def __init__(self,speed):
        self.ss = Pin(16,Pin.OUT)
        self.ss.on()
        self.port = SPI(1,speed)
        self._frame = bytes()

    def _build(self,action,data):
        self._frame = bytes([SYNC1,SYNC2,action,0])
        self._frame = self._frame + bytes[data]
    
    def _send(self):
        self.ss.off()
        self.port.write(self._frame)
        self.ss.on()
    
    def send(self,action,data):
        self.build(action,data)
        self._send()
    
    def hello(self,):
        data = 1 # mapped dataset
        self.send(FRAME_HELLO,data)
    
    def xy(self,d1,d2):
        data = 1 # mapped dataset
        self.send(FRAME_XY,data)
    
    def zt(self,d1,d2):
        data = 1 # mapped dataset
        self.send(FRAME_ZT,data)
    
    def callibrate(self,):
        data = 1 # mapped dataset
        self.send(FRAME_CALLIBRATE,data)
    
    def display(self,d1):
        data = 1 # mapped dataset
        self.send(FRAME_DISPLAY,data)
    
    def fail(self,):
        data = 1 # mapped dataset
        self.send(FRAME_FAIL,data)
    