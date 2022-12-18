# This is the interface to ardiuno minibrain
# this drive the wheels and the 5V sensors

# connection
# green , pin 12 , MISO
# yellow , pin 13 , MOSI
# white , pin 14 , CLK
# blue , pin 27 , SS

from machine import Pin, SPI
import time
import struct

# 20220614 - drive rework
# Convert mini_brain to a frame based comm
# sync , sync ,check, command , d1,d2,d3,d4
# rewrite the Arduino software first (almost dne

# FRAME type enum in newdrive/src/commands.rs

FRAME_HELLO = 0
FRAME_STOP = 1
FRAME_RUN = 2 
FRAME_SETACC = 3 
FRAME_SETJOY = 4 
FRAME_SETTIMEOUT = 5 
FRAME_SETTRIGGER = 6 
FRAME_SETMINSPEED = 7
FRAME_MAXCUR = 8
FRAME_CONFIG = 9 
FRAME_COUNT = 10
# Sync bytes for the frame
SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8

class diff_drive:
    def __init__(self, speed=5000):
        self.ss = Pin(27, Pin.OUT)
        self.ss.on()
        self.port = SPI(1, speed)
        self._frame = bytes([0]*FRAME_SIZE)
        self._rate = 100
        self.accel(15)
        self.timeout(15)
        # incoming interval
        self.interval = 50
        
    def build(self,action,data):
        self._frame  = bytes([SYNC1,SYNC2,action,0])
        self._frame  = self._frame + bytes(data)

    def send(self,action,data):
        self.build(action,data)
        self._send(self._frame)

    def _send(self,fr):
        self.ss.off()
        self.port.write(fr)
        self.ss.on()
    
    def _sr(self):
        outdata = bytearray([0]*FRAME_SIZE)
        self.ss.off()
        self.port.write_readinto(self._frame,outdata)
        self.ss.on()
        return outdata

    def rate(self,val):
        self._rate = val 
    
    def i16_2(self,m1,m2):
        return struct.pack('2h',m1,m2)

    def i16_1(self,m1):
        return self.i16_2(m1,0)
    

    def u8_4(self,u1,u2,u3,u4):
        return struct.pack('4B',u1,u2,u3,u4)

    def u8_1(self,u1):
        return self.u8_4(u1,0,0,0)

    def hello(self):
        self.send(FRAME_HELLO,[0,0,0,0])
    
    def stop(self):
        self.send(FRAME_STOP,[0,0,0,0])
    
    def accel(self,val):
        self.send(FRAME_SETACC,self.u8_1(val))

    def timeout(self,val):
        self.send(FRAME_SETTIMEOUT,self.i16_1(val))

    def maxc(self,val):
        self.send(FRAME_MAXCUR,self.u8_1(val))

    def move(self,m1,m2):
        data = self.i16_2(m1,m2)
        self.send(FRAME_RUN,data)

    def joy(self,m1,m2):
        self.send(FRAME_SETJOY,self.i16_2(m1,m2))

    def forward(self):
        self.move(self._rate, self._rate)

    def backward(self):
        self.move(-self._rate, -self._rate)

    def left(self):
        self.move(-self._rate, self._rate)

    def right(self):
        self.move(self._rate, -self._rate)

    def bounce(self,count=5,timeout=0.5):
        for i in range(count):
            self.forward()
            time.sleep(timeout)
            self.backward()
            time.sleep(timeout)
        self.move(0,0)
        self.stop()
