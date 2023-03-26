# Main runner

#import uasyncio
#import _thread
import struct 

import joycontrol
js = joycontrol.controller(5888)

def count(val=200):
    for i in range(val):
        js.send(joycontrol.FRAME_DISPLAY,struct.pack('i',i))

# Run the telnet server
def run_telnet():
    if reg.telnet:
        print("Starting telnet server")
        import utelnetserver
        utelnetserver.start()

print(reg.id + " Running")
run_telnet()
