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
FRAME_SHOWCAL = 3
FRAME_STARTCAL = 4
FRAME_ENDCAL = 5
FRAME_RESETCAL = 6
FRAME_LOADCAL = 7
FRAME_DISPLAY = 8
FRAME_BRIGHTNESS = 9
FRAME_CLEAR = 10
FRAME_OUTCONTROL = 11
FRAME_OUTSWITCHES = 12
FRAME_DUMPEEPROM = 13
FRAME_ERASEEEPROM = 14
FRAME_LOGGER = 15
FRAME_FAIL = 16

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
        struct.pack_into('',self._data,0,)
        self.send(FRAME_HELLO,self._data)
    
    def xy(self,d1,d2):
        struct.pack_into('bb',self._data,0,d1,d2)
        self.send(FRAME_XY,self._data)
    
    def zt(self,d1,d2):
        struct.pack_into('bb',self._data,0,d1,d2)
        self.send(FRAME_ZT,self._data)
    
    def showcal(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_SHOWCAL,self._data)
    
    def startcal(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_STARTCAL,self._data)
    
    def endcal(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_ENDCAL,self._data)
    
    def resetcal(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_RESETCAL,self._data)
    
    def loadcal(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_LOADCAL,self._data)
    
    def display(self,d1):
        struct.pack_into('i',self._data,0,d1)
        self.send(FRAME_DISPLAY,self._data)
    
    def brightness(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self.send(FRAME_BRIGHTNESS,self._data)
    
    def clear(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_CLEAR,self._data)
    
    def outcontrol(self,d1,d2,d3,d4):
        struct.pack_into('bbbb',self._data,0,d1,d2,d3,d4)
        self.send(FRAME_OUTCONTROL,self._data)
    
    def outswitches(self,d1):
        struct.pack_into('b',self._data,0,d1)
        self.send(FRAME_OUTSWITCHES,self._data)
    
    def dumpeeprom(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_DUMPEEPROM,self._data)
    
    def eraseeeprom(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_ERASEEEPROM,self._data)
    
    def logger(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_LOGGER,self._data)
    
    def fail(self,):
        struct.pack_into('',self._data,0,)
        self.send(FRAME_FAIL,self._data)
    