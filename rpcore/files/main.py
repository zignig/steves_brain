from machine import Pin

bl = Pin(0,Pin.OUT)
us = Pin(1,Pin.OUT)

def bootloader():
    bl.on()
    bl.off()

def userimage():
    us.on()
    us.off()

# main file for rpcore

if reg.telnet:
    import utelnetserver 
    utelnetserver.start()


