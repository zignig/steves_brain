# generated from rust enum


from machine import Pin, SPI , SoftSPI
import time
import struct



SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 

class com:
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
        self.interval = 5
        self.ss.on()
        self.port = SoftSPI(baudrate=speed,sck=Pin(1),mosi=Pin(12),miso=Pin(10))
        self._frame = bytearray([0]*FRAME_SIZE)
        self._return_frame = bytearray([0]*FRAME_SIZE)
        self._data = bytearray([0,0,0,0])
        self._callbacks()

    def _build(self,action,data):
        struct.pack_into(self.data_format[action],self._data,0,*data)
        self._frame = bytearray([SYNC1,SYNC2,0,action])
        self._frame = self._frame + bytes(self._data)
    
    def _send_to_port(self):
        self.ss.off()
        self.port.write(self._frame)
        self.ss.on()
    
    def _read(self):
        self.ss.off()
        data = self.port.read(8)
        self.ss.on()
        self._return_frame = data
        return self._process()
    
    def _send(self,action,data):
        self._build(action,data)
        self._send_to_port()
        time.sleep_ms(self.interval)
        return self._read()
    
    
    def hello(self,):
        return self._send(com.FRAME_HELLO,[])
    
    def start(self,):
        return self._send(com.FRAME_START,[])
    
    def stop(self,):
        return self._send(com.FRAME_STOP,[])
    
    def one(self,d1):
        return self._send(com.FRAME_ONE,[d1])
    
    def two(self,d1,d2):
        return self._send(com.FRAME_TWO,[d1,d2])
    
    def three(self,d1,d2,d3):
        return self._send(com.FRAME_THREE,[d1,d2,d3])
    
    def four(self,d1):
        return self._send(com.FRAME_FOUR,[d1])
    
    def stuff(self,d1):
        return self._send(com.FRAME_STUFF,[d1])
    
    def other(self,d1):
        return self._send(com.FRAME_OTHER,[d1])
    

    def _callbacks(self):
        self.names = ["hello","start","stop","one","two","three","four","stuff","other",]
        self.functions = [None,None,None,None,None,None,None,None,None,]
        self.data_format = ["","","","B","BB","bbb","i","i","I",]

    def bind(self,name,func):
        for i in enumerate(self.names):
            if self.names[i[0]] == name:
                self.functions[i[0]] = func
    
    def _process(self):
        if ((self._return_frame[0] == SYNC1) & (self._return_frame[1] == SYNC2)):
            command = self._return_frame[3]
            data = self._return_frame[4:]
            if self.functions[command] != None:
                up = struct.unpack_from(self.data_format[command],data,0)
                return self.functions[command](*up)