# This is the interface to ardiuno minibrain
# this drive the wheels and the 5V sensors

# connection
# green , pin 12 , MISO
# yellow , pin 13 , MOSI
# white , pin 14 , CLK
# blue , pin 27 , SS

from machine import Pin, SPI
import time

# 20220614 - drive rework
# Convert mini_brain to a frame based comm
# sync , sync , command , d1,d2,d3,d4 check
# rewrite the Arduino software first


class Frame:
    "simple dataframe for arduino comms"
    sync1 = 0xF
    sync2 = 0xE

    def __init__(self):
        self.action = 0
        self.checksum = 0
        self.data1 = 0
        self.data2 = 0
        self.data3 = 0
        self.data4 = 0

    def get(self):
        return bytearray(
            [
                self.sync1,
                self.sync2,
                self.action,
                self.cs(),
                self.data1,
                self.data2,
                self.data3,
                self.data4,
            ]
        )


    def set(self,action,data1=None,data2=None,data3=None,data4=None):
        self.action = action
        if data1 is not None:
            self.data1 = data1
        if data2 is not None:
            self.data2 = data2
        if data3 is not None:
            self.data3 = data3
        if data4 is not None:
            self.data4 = data4

    def cs(self):
        val = (self.data1 + self.data2 + self.data3 + self.data4) % 256
        return val

class diff_drive:
    def __init__(self, speed=10000):
        self.ss = Pin(27, Pin.OUT)
        self.ss.on()
        self.port = SPI(1, speed)
        self.frame = Frame()
        self.rate = 255
        self.accel(200)
        self.interval = 100000

    def _char(self,c):
        self.ss.off()
        self.port.write(c)
        self.ss.on()

    def hello(self):
        self.frame.set(0)
        self._char(self.frame.get())

    def accel(self,acc):
        if ( acc > 255 ) or (acc<= 0):
            raise Exception("accleration out of range")
        self.frame.set(3,acc)
        self._char(self.frame.get())


    def joy(self,m1,m2):
        if ( m1 > 255 ) or (m1 < -255):
            print("motor 1 out of range")
            return
        if ( m2 > 255 ) or (m2 < -255):
            print("motor 2 out of range")
            return
        # default to forward
        dir1 = 0
        dir2 = 0
        if m1 < 0:
            m1 = abs(m1)
            dir1 = 1
        if m2 < 0:
            m2 = abs(m2)
            dir2 = 1
        
        self.frame.set(4,m1,m2,dir1,dir2)
        self._char(self.frame.get())

    def move(self,m1,m2):
        if ( m1 > 255 ) or (m1 < -255):
            raise Exception("motor 1 out of range")
        if ( m2 > 255 ) or (m2 < -255):
            raise Exception("motor 2 out of range")
        # default to forward
        dir1 = 0
        dir2 = 0
        if m1 < 0:
            m1 = abs(m1)
            dir1 = 1
        if m2 < 0:
            m2 = abs(m2)
            dir2 = 1
        self.frame.set(2,m1,m2,dir1,dir2)
        self._char(self.frame.get())

    def stop(self):
        self.frame.set(2,0,0,0,0)
        self._char(self.frame.get())

    def forward(self):
        self.move(self.rate,self.rate)

    def backward(self):
        self.move(-self.rate,-self.rate)

    def left(self):
        self.move(-self.rate,self.rate)

    def right(self):
        self.move(self.rate,-self.rate)
