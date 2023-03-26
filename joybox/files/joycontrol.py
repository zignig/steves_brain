# generated from rust enum

from machine import Pin, SPI
import time
import struct

SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


FRAME_HELLO = 0
FRAME_XY = 1
FRAME_ZT = 2
FRAME_CALLIBRATE = 3
FRAME_DISPLAY = 4
FRAME_BRIGHTNESS = 5
FRAME_CLEAR = 6
FRAME_FAIL = 7

class controller:
    def __init__(self,speed=5000):
        self.ss = Pin(16,Pin.OUT)
        self.ss.on()
        self.port = SPI(1,speed)
        self._frame = bytes([0]*FRAME_SIZE)
        self._data = bytearray([0,0,0,0])

    def _build(self,action,data):
        self._frame = bytes([SYNC1,SYNC2,0,action])
        self._frame = self._frame + bytes(data)
    
    def _send(self):
        self.ss.off()
        self.port.write(self._frame)
        self.ss.on()
    
    def send(self,action,data):
        self._build(action,data)
        self._send()
    
    def hello(self,):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_HELLO,data)
    
    def xy(self,d1,d2):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_XY,data)
    
    def zt(self,d1,d2):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_ZT,data)
    
    def callibrate(self,):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_CALLIBRATE,data)
    
    def display(self,d1):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_DISPLAY,data)
    
    def brightness(self,d1):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_BRIGHTNESS,data)
    
    def clear(self,):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_CLEAR,data)
    
    def fail(self,):
        data = 1 
        # mapped dataset
        # struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_FAIL,data)
    