# generated from rust enum


from machine import Pin, SPI , SoftSPI
import time
import struct



SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


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
FRAME_FAIL = 20

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
        self._process()
    
    def _send(self,action,data):
        self._build(action,data)
        self._send_to_port()
        time.sleep_ms(self.interval)
        self._read()
    
    
    def hello(self,):
        self._send(FRAME_HELLO,[])
    
    def runon(self,):
        self._send(FRAME_RUNON,[])
    
    def xy(self,d1,d2):
        self._send(FRAME_XY,[d1,d2])
    
    def zt(self,d1,d2):
        self._send(FRAME_ZT,[d1,d2])
    
    def showcal(self,):
        self._send(FRAME_SHOWCAL,[])
    
    def startcal(self,):
        self._send(FRAME_STARTCAL,[])
    
    def endcal(self,):
        self._send(FRAME_ENDCAL,[])
    
    def resetcal(self,):
        self._send(FRAME_RESETCAL,[])
    
    def loadcal(self,):
        self._send(FRAME_LOADCAL,[])
    
    def loaddefault(self,):
        self._send(FRAME_LOADDEFAULT,[])
    
    def getmillis(self,d1):
        self._send(FRAME_GETMILLIS,[d1])
    
    def display(self,d1):
        self._send(FRAME_DISPLAY,[d1])
    
    def hexdisplay(self,d1):
        self._send(FRAME_HEXDISPLAY,[d1])
    
    def brightness(self,d1):
        self._send(FRAME_BRIGHTNESS,[d1])
    
    def clear(self,):
        self._send(FRAME_CLEAR,[])
    
    def outcontrol(self,d1,d2,d3,d4):
        self._send(FRAME_OUTCONTROL,[d1,d2,d3,d4])
    
    def outswitches(self,d1):
        self._send(FRAME_OUTSWITCHES,[d1])
    
    def dumpeeprom(self,):
        self._send(FRAME_DUMPEEPROM,[])
    
    def eraseeeprom(self,d1):
        self._send(FRAME_ERASEEEPROM,[d1])
    
    def logger(self,):
        self._send(FRAME_LOGGER,[])
    
    def fail(self,):
        self._send(FRAME_FAIL,[])
    

    def _callbacks(self):
        self.names = ["hello","runon","xy","zt","showcal","startcal","endcal","resetcal","loadcal","loaddefault","getmillis","display","hexdisplay","brightness","clear","outcontrol","outswitches","dumpeeprom","eraseeeprom","logger","fail",]
        self.functions = [None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,None,]
        self.data_format = ["","","bb","bb","","","","","","","i","I","i","B","","bbbb","b","","B","","",]

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