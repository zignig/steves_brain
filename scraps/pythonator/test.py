# generated from rust enum

from machine import Pin, SPI
import time
import struct



FRAME_HELLO = 0
FRAME_START = 1
FRAME_STOP = 2
FRAME_JOY = 3
FRAME_THROTTLE = 4
FRAME_ONE = 5
FRAME_TWO = 6
FRAME_THREE = 7
FRAME_FIVE = 8
FRAME_SIZE = 9

class controller:
    def __init__(self,speed):
        self.ss = Pin(27,Pin.OUT)
        self.ss.on()
        self.port = SPI(1,speed)
        self._frame = bytes()

    def _build(self,action,data):
        self._frame = bytes([SYNC1,SYNC2,action,0])
        self._frame = self._frame + bytes[data]
    
    def _send(self):
        self.ss.off()
        self.port.write(self._frame)
        self.ss.off()
    
    def send(self,action,data):
        self.build(action,data)
        self._send()
    
    def hello(self,):
        data = 1 # mapped dataset
        self.send(FRAME_HELLO,data)
    
    def start(self,):
        data = 1 # mapped dataset
        self.send(FRAME_START,data)
    
    def stop(self,):
        data = 1 # mapped dataset
        self.send(FRAME_STOP,data)
    
    def joy(self,d1,d2,d3):
        data = 1 # mapped dataset
        self.send(FRAME_JOY,data)
    
    def throttle(self,d1):
        data = 1 # mapped dataset
        self.send(FRAME_THROTTLE,data)
    
    def one(self,):
        data = 1 # mapped dataset
        self.send(FRAME_ONE,data)
    
    def two(self,):
        data = 1 # mapped dataset
        self.send(FRAME_TWO,data)
    
    def three(self,d1,d2,d3):
        data = 1 # mapped dataset
        self.send(FRAME_THREE,data)
    
    def five(self,d1):
        data = 1 # mapped dataset
        self.send(FRAME_FIVE,data)
    
    def size(self,d1,d2):
        data = 1 # mapped dataset
        self.send(FRAME_SIZE,data)
    