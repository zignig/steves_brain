# mock interface for interface testing 
import copy

class Pin:
    OUT = 1
    def __init__(self,pin_number,mode):
        self.pin = pin_number
        self.mode = mode
        self.value = 0

    def off(self):
        self.value = 0

    def on(self):
        self.value = 1
    
class SPI:
    def __init__(self,i,speed):
        self.i = i
        self.speed = speed

    def write(self,data):
        print(data)
        pass

    def write_readinto(self,frame,return_frame):
        return_frame[:] = frame.copy()
        print(frame,return_frame)