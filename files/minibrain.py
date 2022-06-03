# This is the interface to ardiuno minibrain
# this drive the wheels and the 5V sensors
from machine import UART

class diff_drive:
    def __init__(self,uart=2,speed=9600):
        self.port = UART(uart,speed)
        
    
    def forward(self):
        self.port.write('w')

    def backward(self):
        self.port.write('s')

    def left(self):
        self.port.write('a')

    def right(self):
        self.port.write('d')

    def read(self):
        while True:
            val = self.port.read()
            if val is not None:
                print(val) 
