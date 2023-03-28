# generated from rust enum

from machine import Pin, SPI
import time
import struct

SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


FRAME_HELLO = 0
FRAME_STOP = 1
FRAME_RUN = 2
FRAME_SETACC = 3
FRAME_SETJOY = 4
FRAME_SETTIMEOUT = 5
FRAME_SETTRIGGER = 6
FRAME_SETMINSPEED = 7
FRAME_SETMAXCURRENT = 8
FRAME_CONFIG = 9
FRAME_COUNT = 10
FRAME_DATA = 11
FRAME_COMPASS = 12
FRAME_MILLIS = 13
FRAME_FAIL = 14

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
    
    def stop(self,):
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_STOP,data)
    
    def run(self,d1,d2):
        data  = struct.pack_into('HH',self._data,0,d1,d2)
        self.send(FRAME_RUN,data)
    
    def setacc(self,d1):
        data  = struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_SETACC,data)
    
    def setjoy(self,d1,d2):
        data  = struct.pack_into('HH',self._data,0,d1,d2)
        self.send(FRAME_SETJOY,data)
    
    def settimeout(self,d1):
        data  = struct.pack_into('H',self._data,0,d1)
        self.send(FRAME_SETTIMEOUT,data)
    
    def settrigger(self,d1):
        data  = struct.pack_into('H',self._data,0,d1)
        self.send(FRAME_SETTRIGGER,data)
    
    def setminspeed(self,d1):
        data  = struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_SETMINSPEED,data)
    
    def setmaxcurrent(self,d1):
        data  = struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_SETMAXCURRENT,data)
    
    def config(self,):
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_CONFIG,data)
    
    def count(self,):
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_COUNT,data)
    
    def data(self,d1,d2,d3,d4):
        data  = struct.pack_into('BBBB',self._data,0,d1,d2,d3,d4)
        self.send(FRAME_DATA,data)
    
    def compass(self,d1):
        data  = struct.pack_into('H',self._data,0,d1)
        self.send(FRAME_COMPASS,data)
    
    def millis(self,d1):
        data  = struct.pack_into('I',self._data,0,d1)
        self.send(FRAME_MILLIS,data)
    
    def fail(self,):
        data  = struct.pack_into('',self._data,0,)
        self.send(FRAME_FAIL,data)
    