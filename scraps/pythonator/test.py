# generated from rust enum

from machine import Pin, SPI
import time
import struct

SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


FRAME_HELLO = 0
FRAME_START = 1
FRAME_STOP = 2
FRAME_JOY = 3
FRAME_THROTTLE = 4
FRAME_CALLIBRATE = 5
FRAME_DISPLAY = 6
FRAME_FNORD = 7
FRAME_CROSS = 8

class controller:
    def __init__(self,speed=5000):
        self.ss = Pin(27,Pin.OUT)
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
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_HELLO,data)
    
    def start(self,):
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_START,data)
    
    def stop(self,):
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_STOP,data)
    
    def joy(self,d1,d2,d3):
        data  = struct.pack_into('bbb',self._data,0,d1,d2,d3)
        self.send(FRAME_JOY,data)
    
    def throttle(self,d1):
        data  = struct.pack_into('b',self._data,0,d1)
        self.send(FRAME_THROTTLE,data)
    
    def callibrate(self,d1):
        data  = struct.pack_into('b',self._data,0,d1)
        self.send(FRAME_CALLIBRATE,data)
    
    def display(self,d1,d2):
        data  = struct.pack_into('BB',self._data,0,d1,d2)
        self.send(FRAME_DISPLAY,data)
    
    def fnord(self,d1):
        data  = struct.pack_into('i',self._data,0,d1)
        self.send(FRAME_FNORD,data)
    
    def cross(self,d1,d2,d3):
        data  = struct.pack_into('BHb',self._data,0,d1,d2,d3)
        self.send(FRAME_CROSS,data)
    