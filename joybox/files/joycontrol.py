# generated from rust enum


from machine import Pin, SPI
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
    def __init__(self, speed=10000):
        self.ss = Pin(16, Pin.OUT)
        self.ss.on()
        self.port = SPI(1, speed)
        self._frame = bytearray([0] * FRAME_SIZE)
        self._return_frame = bytearray([0] * FRAME_SIZE)
        self._data = bytearray([0, 0, 0, 0])
        self._callbacks()

    def _build(self, action, data):
        self._frame = bytearray([SYNC1, SYNC2, 0, action])
        self._frame = self._frame + bytes(data)

    def _send_to_port(self):
        #print("write")
        self.ss.off()
        # self.port.write(self._frame)
        # print(list(self._frame))
        self.port.write(self._frame)
        self.ss.on()

    def _read(self):
        #print("read")
        self.ss.off()
        self._return_frame = self.port.read(8)
        self.ss.on()
        self._process()

    def _send(self, action, data):
        self._build(action, data)
        self._send_to_port()
        #time.sleep_ms(90)
        self._read()
        # read ??

    def hello(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_HELLO, self._data)

    def runon(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_RUNON, self._data)

    def xy(self, d1, d2):
        struct.pack_into("bb", self._data, 0, d1, d2)
        self._send(FRAME_XY, self._data)

    def zt(self, d1, d2):
        struct.pack_into("bb", self._data, 0, d1, d2)
        self._send(FRAME_ZT, self._data)

    def showcal(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_SHOWCAL, self._data)

    def startcal(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_STARTCAL, self._data)

    def endcal(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_ENDCAL, self._data)

    def resetcal(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_RESETCAL, self._data)

    def loadcal(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_LOADCAL, self._data)

    def loaddefault(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_LOADDEFAULT, self._data)

    def getmillis(self, d1):
        struct.pack_into("i", self._data, 0, d1)
        self._send(FRAME_GETMILLIS, self._data)

    def display(self, d1):
        struct.pack_into("I", self._data, 0, d1)
        self._send(FRAME_DISPLAY, self._data)

    def hexdisplay(self, d1):
        struct.pack_into("i", self._data, 0, d1)
        self._send(FRAME_HEXDISPLAY, self._data)

    def brightness(self, d1):
        struct.pack_into("B", self._data, 0, d1)
        self._send(FRAME_BRIGHTNESS, self._data)

    def clear(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_CLEAR, self._data)

    def outcontrol(self, d1, d2, d3, d4):
        struct.pack_into("bbbb", self._data, 0, d1, d2, d3, d4)
        self._send(FRAME_OUTCONTROL, self._data)

    def outswitches(self, d1):
        struct.pack_into("b", self._data, 0, d1)
        self._send(FRAME_OUTSWITCHES, self._data)

    def dumpeeprom(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_DUMPEEPROM, self._data)

    def eraseeeprom(self, d1):
        struct.pack_into("B", self._data, 0, d1)
        self._send(FRAME_ERASEEEPROM, self._data)

    def logger(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_LOGGER, self._data)

    def fail(
        self,
    ):
        struct.pack_into(
            "",
            self._data,
            0,
        )
        self._send(FRAME_FAIL, self._data)

    def _callbacks(self):
        self.names = [
            "hello",
            "runon",
            "xy",
            "zt",
            "showcal",
            "startcal",
            "endcal",
            "resetcal",
            "loadcal",
            "loaddefault",
            "getmillis",
            "display",
            "hexdisplay",
            "brightness",
            "clear",
            "outcontrol",
            "outswitches",
            "dumpeeprom",
            "eraseeeprom",
            "logger",
            "fail",
        ]
        self.functions = [
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ]
        self.data_format = [
            "",
            "",
            "bb",
            "bb",
            "",
            "",
            "",
            "",
            "",
            "",
            "i",
            "I",
            "i",
            "B",
            "",
            "bbbb",
            "b",
            "",
            "B",
            "",
            "",
        ]

    def bind(self, name, func):
        for i in enumerate(self.names):
            if self.names[i[0]] == name:
                self.functions[i[0]] = func

    def _process(self):
        command = self._return_frame[3]
        data = self._return_frame[4:]
        #print(list(self._return_frame))
        if command < len(self.functions):
            if self.functions[command] != None:
                up = struct.unpack_from(self.data_format[command], data, 0)
                self.functions[command](*up)
