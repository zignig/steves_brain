# generated from rust enum


from machine import Pin, SPI
import time
import struct



SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


FRAME_HELLO = 0
FRAME_STOP = 1
FRAME_RUN = 2
FRAME_SETACC = 3
FRAME_SETJOY = 4
FRAME_SETTIMEOUT = 5
FRAME_SETTRIGGER = 6
FRAME_SETMINSPEED = 7
FRAME_SETMAXCURRENT = 8
FRAME_CONFIG = 9
FRAME_COUNT = 10
FRAME_DATA = 11
FRAME_COMPASS = 12
FRAME_MILLIS = 13
FRAME_FAIL = 14

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
    
    def stop(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_STOP,self._data)
    
    def run(self,d1,d2):
        struct.pack_into('HH',self._data,0,d1,d2)
        self._send(FRAME_RUN,self._data)
    
    def setacc(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self._send(FRAME_SETACC,self._data)
    
    def setjoy(self,d1,d2):
        struct.pack_into('HH',self._data,0,d1,d2)
        self._send(FRAME_SETJOY,self._data)
    
    def settimeout(self,d1):
        struct.pack_into('H',self._data,0,d1)
        self._send(FRAME_SETTIMEOUT,self._data)
    
    def settrigger(self,d1):
        struct.pack_into('H',self._data,0,d1)
        self._send(FRAME_SETTRIGGER,self._data)
    
    def setminspeed(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self._send(FRAME_SETMINSPEED,self._data)
    
    def setmaxcurrent(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self._send(FRAME_SETMAXCURRENT,self._data)
    
    def config(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_CONFIG,self._data)
    
    def count(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_COUNT,self._data)
    
    def data(self,d1,d2,d3,d4):
        struct.pack_into('BBBB',self._data,0,d1,d2,d3,d4)
        self._send(FRAME_DATA,self._data)
    
    def compass(self,d1):
        struct.pack_into('H',self._data,0,d1)
        self._send(FRAME_COMPASS,self._data)
    
    def millis(self,d1):
        struct.pack_into('I',self._data,0,d1)
        self._send(FRAME_MILLIS,self._data)
    
    def fail(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_FAIL,self._data)
    

    def _callbacks(self):
        self.names = ["hello","stop","run","setacc","setjoy","settimeout","settrigger","setminspeed","setmaxcurrent","config","count","data","compass","millis","fail",]
        self.functions = [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]
        self.data_format = ["","","HH","B","HH","H","H","B","B","","","BBBB","H","I","",]

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