# generated from rust enum


from machine import Pin, SPI , SoftSPI
import time
import struct

SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 

class com:
    FRAME_HELLO = 0
    FRAME_RUNON = 1
    FRAME_XY = 2
    FRAME_ZT = 3
    FRAME_SHOWCAL = 4
    FRAME_STARTCAL = 5
    FRAME_ENDCAL = 6
    FRAME_RESETCAL = 7
    FRAME_LOADCAL = 8
    FRAME_LOADDEFAULT = 9
    FRAME_GETMILLIS = 10
    FRAME_DISPLAY = 11
    FRAME_HEXDISPLAY = 12
    FRAME_BRIGHTNESS = 13
    FRAME_CLEAR = 14
    FRAME_OUTCONTROL = 15
    FRAME_OUTSWITCHES = 16
    FRAME_DUMPEEPROM = 17
    FRAME_ERASEEEPROM = 18
    FRAME_LOGGER = 19
    FRAME_VERBOSE = 20
    FRAME_LEFTBUTTON = 21
    FRAME_RIGHTBUTTON = 22
    FRAME_ESTOP = 23
    FRAME_MISSILE = 24
    FRAME_FAIL = 25

# create the controller device
class controller:
    def __init__(self,speed=10000):
        self.ss = Pin(16,Pin.OUT)
        self.interval = 20
        self.ss.on()
        self.port = SoftSPI(baudrate=speed,sck=Pin(14),mosi=Pin(13),miso=Pin(12))
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
    
    def runon(self,):
        return self._send(com.FRAME_RUNON,[])
    
    def xy(self,d1,d2):
        return self._send(com.FRAME_XY,[d1,d2])
    
    def zt(self,d1,d2):
        return self._send(com.FRAME_ZT,[d1,d2])
    
    def showcal(self,):
        return self._send(com.FRAME_SHOWCAL,[])
    
    def startcal(self,):
        return self._send(com.FRAME_STARTCAL,[])
    
    def endcal(self,):
        return self._send(com.FRAME_ENDCAL,[])
    
    def resetcal(self,):
        return self._send(com.FRAME_RESETCAL,[])
    
    def loadcal(self,):
        return self._send(com.FRAME_LOADCAL,[])
    
    def loaddefault(self,):
        return self._send(com.FRAME_LOADDEFAULT,[])
    
    def getmillis(self,d1):
        return self._send(com.FRAME_GETMILLIS,[d1])
    
    def display(self,d1):
        return self._send(com.FRAME_DISPLAY,[d1])
    
    def hexdisplay(self,d1):
        return self._send(com.FRAME_HEXDISPLAY,[d1])
    
    def brightness(self,d1):
        return self._send(com.FRAME_BRIGHTNESS,[d1])
    
    def clear(self,):
        return self._send(com.FRAME_CLEAR,[])
    
    def outcontrol(self,d1,d2,d3,d4):
        return self._send(com.FRAME_OUTCONTROL,[d1,d2,d3,d4])
    
    def outswitches(self,d1):
        return self._send(com.FRAME_OUTSWITCHES,[d1])
    
    def dumpeeprom(self,):
        return self._send(com.FRAME_DUMPEEPROM,[])
    
    def eraseeeprom(self,d1):
        return self._send(com.FRAME_ERASEEEPROM,[d1])
    
    def logger(self,):
        return self._send(com.FRAME_LOGGER,[])
    
    def verbose(self,):
        return self._send(com.FRAME_VERBOSE,[])
    
    def leftbutton(self,):
        return self._send(com.FRAME_LEFTBUTTON,[])
    
    def rightbutton(self,):
        return self._send(com.FRAME_RIGHTBUTTON,[])
    
    def estop(self,):
        return self._send(com.FRAME_ESTOP,[])
    
    def missile(self,):
        return self._send(com.FRAME_MISSILE,[])
    
    def fail(self,):
        return self._send(com.FRAME_FAIL,[])
    

    def _callbacks(self):
        self.names = ["hello","runon","xy","zt","showcal","startcal","endcal","resetcal","loadcal","loaddefault","getmillis","display","hexdisplay","brightness","clear","outcontrol","outswitches","dumpeeprom","eraseeeprom","logger","verbose","leftbutton","rightbutton","estop","missile","fail",]
        self.functions = [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]
        self.data_format = ["","","bb","hh","","","","","","","I","i","I","B","","bbbb","b","","B","","","","","","","",]

    def bind(self,name,func):
        for i in enumerate(self.names):
            if self.names[i[0]] == name:
                self.functions[i[0]] = func
                return
        
    def _process(self):
        if ((self._return_frame[0] == SYNC1) & (self._return_frame[1] == SYNC2)):
            command = self._return_frame[3]
            data = self._return_frame[4:]
            if self.functions[command] != None:
                up = struct.unpack_from(self.data_format[command],data,0)
                return self.functions[command](*up)