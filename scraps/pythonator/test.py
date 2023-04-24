# generated from rust enum


from machine import Pin, SPI
import time
import struct



SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


FRAME_HELLO = 0
FRAME_START = 1
FRAME_STOP = 2
FRAME_ONE = 3
FRAME_TWO = 4
FRAME_THREE = 5
FRAME_FOUR = 6
FRAME_STUFF = 7
FRAME_OTHER = 8

# create the controller device
class controller:
    def __init__(self,speed=10000):
        self.ss = Pin(27,Pin.OUT)
        self.ss.on()
        self.port = SPI(1,speed)
        self._frame = bytearray([0]*FRAME_SIZE)
        self._return_frame = bytearray([0]*FRAME_SIZE)
        self._data = bytearray([0,0,0,0])
        self._callbacks()

    def _build(self,action,data):
        self._frame = bytearray([SYNC1,SYNC2,0,action])
        self._frame = self._frame + bytes(data)
    
    def _send_to_port(self):
        self.ss.off()
        # self.port.write(self._frame)
        self.port.write_readinto(self._frame,self._return_frame)
        self.ss.on()
    
    def _send(self,action,data):
        self._build(action,data)
        self._send_to_port()
        self._process()
    
    def hello(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_HELLO,self._data)
    
    def start(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_START,self._data)
    
    def stop(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_STOP,self._data)
    
    def one(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self._send(FRAME_ONE,self._data)
    
    def two(self,d1,d2):
        struct.pack_into('BB',self._data,0,d1,d2)
        self._send(FRAME_TWO,self._data)
    
    def three(self,d1,d2,d3):
        struct.pack_into('bbb',self._data,0,d1,d2,d3)
        self._send(FRAME_THREE,self._data)
    
    def four(self,d1):
        struct.pack_into('i',self._data,0,d1)
        self._send(FRAME_FOUR,self._data)
    
    def stuff(self,d1):
        struct.pack_into('i',self._data,0,d1)
        self._send(FRAME_STUFF,self._data)
    
    def other(self,d1):
        struct.pack_into('I',self._data,0,d1)
        self._send(FRAME_OTHER,self._data)
    

    def _callbacks(self):
        self.names = ["hello","start","stop","one","two","three","four","stuff","other",]
        self.functions = [None,None,None,None,None,None,None,None,None,]
        self.data_format = ["","","","B","BB","bbb","i","i","I",]

    def bind(self,name,func):
        for i in enumerate(self.names):
            if self.names[i[0]] == name:
                self.functions[i[0]] = func
    
    def _process(self):
        command = self._return_frame[3]
        data = self._return_frame[4:]
        if self.functions[command] != None:
            up = struct.unpack_from(self.data_format[command],data,0)
            self.functions[command](*up)