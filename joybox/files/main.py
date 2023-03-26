# Main runner

#import uasyncio
#import _thread

import joycontrol
js = joycontrol.controller(5888)

# Run the telnet server
def run_telnet():
    if reg.telnet:
        print("Starting telnet server")
        import utelnetserver
        utelnetserver.start()

print(reg.id + " Running")
run_telnet()
