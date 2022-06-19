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
        print(val)
        return val

class diff_drive:
    def __init__(self, speed=10000):
        self.ss = Pin(27, Pin.OUT)
        self.ss.on()
        self.port = SPI(1, speed)
        self.frame = Frame()

    def _send(self, s):
        self.ss.off()
        self.port.write(s + "\n")
        self.ss.on()

    def _char(self,c):
        self.ss.off()
        self.port.write(c)
        self.ss.on()

    def _loop(self, s):
        z = bytearray(len(s) + 1)
        self.ss.off()
        self.port.write_readinto(s + "\n", z)
        self.ss.on()
        return z

    def go(self,action):
        self.frame.set(action)
        self._char(self.frame.get())

    def forward(self):
        self._send("w")

    def backward(self):
        self._send("s")

    def left(self):
        self._send("a")

    def right(self):
        self._send("d")

    def faster(self):
        self._send("]")

    def slower(self):
        self._send("[")
