# Main runner

#import uasyncio
#import _thread
import struct 

import joycontrol
js = joycontrol.controller(10000)

def num(val=0):
    js.send(joycontrol.FRAME_DISPLAY,struct.pack('i',val))

def brightness(val=0):
    js.send(joycontrol.FRAME_BRIGHTNESS,[val,0,0,0])

def clear():
    js.send(joycontrol.FRAME_CLEAR,[0,0,0,0])

def count(val=200):
    for i in range(val):
        js.send(joycontrol.FRAME_DISPLAY,struct.pack('i',i+1))

def startcal():
    js.send(joycontrol.FRAME_STARTCAL,[0,0,0,0])

def endcal():
    js.send(joycontrol.FRAME_ENDCAL,[0,0,0,0])

def resetcal():
    js.send(joycontrol.FRAME_RESETCAL,[0,0,0,0])

# Run the telnet server
def run_telnet():
    if reg.telnet:
        print("Starting telnet server")
        import utelnetserver
        utelnetserver.start()

print(reg.id + " Running")
run_telnet()
