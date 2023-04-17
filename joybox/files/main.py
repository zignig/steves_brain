# Main runner

#import uasyncio
#import _thread

import struct 

led = machine.Pin(5,machine.Pin.OUT)
switch1 = machine.Pin(4,machine.Pin.IN)
switch2 = machine.Pin(2  ,machine.Pin.IN)

# Joy control is templated with the pythonator.
 
import joycontrol
js = joycontrol.controller(5000)


def count(val=200):
    for i in range(val):
        js.display(i+1)

def watch_buttons():
    while True:
        print("switch1 - ",switch1.value())
        print("switch2 - ",switch2.value())
        time.sleep_ms(100)
              

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
