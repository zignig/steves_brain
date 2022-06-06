# This is the interface to ardiuno minibrain
# this drive the wheels and the 5V sensors
from machine import Pin,SPI
class diff_drive:
    def __init__(self,speed=10000):
        self.port = SPI(1,speed) 
        self.ss = Pin(27,Pin.OUT)
        self.ss.off()
 
    def forward(self):
        self.port.write('w\n')

    def backward(self):
        self.port.write('s\n')

    def left(self):
        self.port.write('a\n')

    def right(self):
        self.port.write('d\n')

