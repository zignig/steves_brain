# generated from rust enum


from machine import Pin, SPI , SoftSPI
import time
import struct



SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 

class com:
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
        self.interval = 30
        self.ss.on()
        self.port = SoftSPI(baudrate=speed,sck=Pin(10),mosi=Pin(11),miso=Pin(12))
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
    
    def stop(self,):
        return self._send(com.FRAME_STOP,[])
    
    def run(self,d1,d2):
        return self._send(com.FRAME_RUN,[d1,d2])
    
    def setacc(self,d1):
        return self._send(com.FRAME_SETACC,[d1])
    
    def setjoy(self,d1,d2):
        return self._send(com.FRAME_SETJOY,[d1,d2])
    
    def settimeout(self,d1):
        return self._send(com.FRAME_SETTIMEOUT,[d1])
    
    def settrigger(self,d1):
        return self._send(com.FRAME_SETTRIGGER,[d1])
    
    def setminspeed(self,d1):
        return self._send(com.FRAME_SETMINSPEED,[d1])
    
    def setmaxcurrent(self,d1):
        return self._send(com.FRAME_SETMAXCURRENT,[d1])
    
    def config(self,):
        return self._send(com.FRAME_CONFIG,[])
    
    def count(self,):
        return self._send(com.FRAME_COUNT,[])
    
    def data(self,d1,d2,d3,d4):
        return self._send(com.FRAME_DATA,[d1,d2,d3,d4])
    
    def compass(self,d1):
        return self._send(com.FRAME_COMPASS,[d1])
    
    def millis(self,d1):
        return self._send(com.FRAME_MILLIS,[d1])
    
    def fail(self,):
        return self._send(com.FRAME_FAIL,[])
    

    def _callbacks(self):
        self.names = ["hello","stop","run","setacc","setjoy","settimeout","settrigger","setminspeed","setmaxcurrent","config","count","data","compass","millis","fail",]
        self.functions = [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]
        self.data_format = ["","","HH","B","HH","H","H","B","B","","","BBBB","H","i","",]

    def bind(self,name,func):
        for i in enumerate(self.names):
            if self.names[i[0]] == name:
                self.functions[i[0]] = func
    
    def _process(self):
        command = self._return_frame[3]
        data = self._return_frame[4:]
        if self.functions[command] != None:
            up = struct.unpack_from(self.data_format[command],data,0)
            return self.functions[command](*up)