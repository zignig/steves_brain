# This is the interface to ardiuno minibrain
# this drive the wheels and the 5V sensors

# connection
# green , pin 12 , MISO
# yellow , pin 13 , MOSI
# white , pin 14 , CLK
# blue , pin 27 , SS

from machine import Pin,SPI
import time

class diff_drive:
    def __init__(self,speed=10000):
        self.ss = Pin(27,Pin.OUT)
        self.ss.on()
        self.port = SPI(1,speed) 
 
    def _send(self,s):
        self.ss.off()
        self.port.write(s+'\n')
        self.ss.on()

    def _loop(self,s):
        z = bytearray(len(s)+1)
        self.ss.off()
        self.port.write_readinto(s+'\n',z)
        self.ss.on()
        return z

    def forward(self):
        self._send('w')

    def backward(self):
        self._send('s')

    def left(self):
        self._send('a')

    def right(self):
        self._send('d')

    def faster(self):
        self._send(']')

    def slower(self):
        self._send('[')

