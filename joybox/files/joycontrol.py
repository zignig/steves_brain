# generated from rust enum


from machine import Pin, SPI
import time
import struct



SYNC1 = 0xF
SYNC2 = 0xE
FRAME_SIZE = 8 


FRAME_HELLO = 0
FRAME_XY = 1
FRAME_ZT = 2
FRAME_SHOWCAL = 3
FRAME_STARTCAL = 4
FRAME_ENDCAL = 5
FRAME_RESETCAL = 6
FRAME_LOADCAL = 7
FRAME_LOADDEFAULT = 8
FRAME_GETMILLIS = 9
FRAME_DISPLAY = 10
FRAME_HEXDISPLAY = 11
FRAME_BRIGHTNESS = 12
FRAME_CLEAR = 13
FRAME_OUTCONTROL = 14
FRAME_OUTSWITCHES = 15
FRAME_DUMPEEPROM = 16
FRAME_ERASEEEPROM = 17
FRAME_LOGGER = 18
FRAME_FAIL = 19

# create the controller device
class controller:
    def __init__(self,speed=10000):
        self.ss = Pin(16,Pin.OUT)
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
    
    def xy(self,d1,d2):
        struct.pack_into('bb',self._data,0,d1,d2)
        self._send(FRAME_XY,self._data)
    
    def zt(self,d1,d2):
        struct.pack_into('bb',self._data,0,d1,d2)
        self._send(FRAME_ZT,self._data)
    
    def showcal(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_SHOWCAL,self._data)
    
    def startcal(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_STARTCAL,self._data)
    
    def endcal(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_ENDCAL,self._data)
    
    def resetcal(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_RESETCAL,self._data)
    
    def loadcal(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_LOADCAL,self._data)
    
    def loaddefault(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_LOADDEFAULT,self._data)
    
    def getmillis(self,d1):
        struct.pack_into('I',self._data,0,d1)
        self._send(FRAME_GETMILLIS,self._data)
    
    def display(self,d1):
        struct.pack_into('i',self._data,0,d1)
        self._send(FRAME_DISPLAY,self._data)
    
    def hexdisplay(self,d1):
        struct.pack_into('I',self._data,0,d1)
        self._send(FRAME_HEXDISPLAY,self._data)
    
    def brightness(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self._send(FRAME_BRIGHTNESS,self._data)
    
    def clear(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_CLEAR,self._data)
    
    def outcontrol(self,d1,d2,d3,d4):
        struct.pack_into('bbbb',self._data,0,d1,d2,d3,d4)
        self._send(FRAME_OUTCONTROL,self._data)
    
    def outswitches(self,d1):
        struct.pack_into('b',self._data,0,d1)
        self._send(FRAME_OUTSWITCHES,self._data)
    
    def dumpeeprom(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_DUMPEEPROM,self._data)
    
    def eraseeeprom(self,d1):
        struct.pack_into('B',self._data,0,d1)
        self._send(FRAME_ERASEEEPROM,self._data)
    
    def logger(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_LOGGER,self._data)
    
    def fail(self,):
        struct.pack_into('',self._data,0,)
        self._send(FRAME_FAIL,self._data)
    

    def _callbacks(self):
        self.cb_hello = None
        self.cb_xy = None
        self.cb_zt = None
        self.cb_showcal = None
        self.cb_startcal = None
        self.cb_endcal = None
        self.cb_resetcal = None
        self.cb_loadcal = None
        self.cb_loaddefault = None
        self.cb_getmillis = None
        self.cb_display = None
        self.cb_hexdisplay = None
        self.cb_brightness = None
        self.cb_clear = None
        self.cb_outcontrol = None
        self.cb_outswitches = None
        self.cb_dumpeeprom = None
        self.cb_eraseeeprom = None
        self.cb_logger = None
        self.cb_fail = None

    def _process(self):
        command = self._return_frame[3]
        data = self._return_frame[4:]
        if command  == FRAME_HELLO:
            up = struct.unpack_from('',data,0)
            if self.cb_hello != None:
                self.cb_hello(*up)
        elif command  == FRAME_XY:
            up = struct.unpack_from('bb',data,0)
            if self.cb_xy != None:
                self.cb_xy(*up)
        elif command  == FRAME_ZT:
            up = struct.unpack_from('bb',data,0)
            if self.cb_zt != None:
                self.cb_zt(*up)
        elif command  == FRAME_SHOWCAL:
            up = struct.unpack_from('',data,0)
            if self.cb_showcal != None:
                self.cb_showcal(*up)
        elif command  == FRAME_STARTCAL:
            up = struct.unpack_from('',data,0)
            if self.cb_startcal != None:
                self.cb_startcal(*up)
        elif command  == FRAME_ENDCAL:
            up = struct.unpack_from('',data,0)
            if self.cb_endcal != None:
                self.cb_endcal(*up)
        elif command  == FRAME_RESETCAL:
            up = struct.unpack_from('',data,0)
            if self.cb_resetcal != None:
                self.cb_resetcal(*up)
        elif command  == FRAME_LOADCAL:
            up = struct.unpack_from('',data,0)
            if self.cb_loadcal != None:
                self.cb_loadcal(*up)
        elif command  == FRAME_LOADDEFAULT:
            up = struct.unpack_from('',data,0)
            if self.cb_loaddefault != None:
                self.cb_loaddefault(*up)
        elif command  == FRAME_GETMILLIS:
            up = struct.unpack_from('I',data,0)
            if self.cb_getmillis != None:
                self.cb_getmillis(*up)
        elif command  == FRAME_DISPLAY:
            up = struct.unpack_from('i',data,0)
            if self.cb_display != None:
                self.cb_display(*up)
        elif command  == FRAME_HEXDISPLAY:
            up = struct.unpack_from('I',data,0)
            if self.cb_hexdisplay != None:
                self.cb_hexdisplay(*up)
        elif command  == FRAME_BRIGHTNESS:
            up = struct.unpack_from('B',data,0)
            if self.cb_brightness != None:
                self.cb_brightness(*up)
        elif command  == FRAME_CLEAR:
            up = struct.unpack_from('',data,0)
            if self.cb_clear != None:
                self.cb_clear(*up)
        elif command  == FRAME_OUTCONTROL:
            up = struct.unpack_from('bbbb',data,0)
            if self.cb_outcontrol != None:
                self.cb_outcontrol(*up)
        elif command  == FRAME_OUTSWITCHES:
            up = struct.unpack_from('b',data,0)
            if self.cb_outswitches != None:
                self.cb_outswitches(*up)
        elif command  == FRAME_DUMPEEPROM:
            up = struct.unpack_from('',data,0)
            if self.cb_dumpeeprom != None:
                self.cb_dumpeeprom(*up)
        elif command  == FRAME_ERASEEEPROM:
            up = struct.unpack_from('B',data,0)
            if self.cb_eraseeeprom != None:
                self.cb_eraseeeprom(*up)
        elif command  == FRAME_LOGGER:
            up = struct.unpack_from('',data,0)
            if self.cb_logger != None:
                self.cb_logger(*up)
        elif command  == FRAME_FAIL:
            up = struct.unpack_from('',data,0)
            if self.cb_fail != None:
                self.cb_fail(*up)