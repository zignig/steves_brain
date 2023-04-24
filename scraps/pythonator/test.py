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
        self.cb_hello = None
        self.cb_start = None
        self.cb_stop = None
        self.cb_one = None
        self.cb_two = None
        self.cb_three = None
        self.cb_four = None
        self.cb_stuff = None
        self.cb_other = None

    def _process(self):
        command = self._return_frame[3]
        data = self._return_frame[4:]
        if command  == FRAME_HELLO:
            up = struct.unpack_from('',data,0)
            if self.cb_hello != None:
                self.cb_hello(*up)
        elif command  == FRAME_START:
            up = struct.unpack_from('',data,0)
            if self.cb_start != None:
                self.cb_start(*up)
        elif command  == FRAME_STOP:
            up = struct.unpack_from('',data,0)
            if self.cb_stop != None:
                self.cb_stop(*up)
        elif command  == FRAME_ONE:
            up = struct.unpack_from('B',data,0)
            if self.cb_one != None:
                self.cb_one(*up)
        elif command  == FRAME_TWO:
            up = struct.unpack_from('BB',data,0)
            if self.cb_two != None:
                self.cb_two(*up)
        elif command  == FRAME_THREE:
            up = struct.unpack_from('bbb',data,0)
            if self.cb_three != None:
                self.cb_three(*up)
        elif command  == FRAME_FOUR:
            up = struct.unpack_from('i',data,0)
            if self.cb_four != None:
                self.cb_four(*up)
        elif command  == FRAME_STUFF:
            up = struct.unpack_from('i',data,0)
            if self.cb_stuff != None:
                self.cb_stuff(*up)
        elif command  == FRAME_OTHER:
            up = struct.unpack_from('I',data,0)
            if self.cb_other != None:
                self.cb_other(*up)