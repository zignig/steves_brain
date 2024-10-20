# Main runner

# Warning this is auto updated
import web_interface

#upip.index_urls = [reg.uplink + '/packages']

import os
import gc

# set the gc threshold quite low
gc.threshold(4096)

def show(directory="/"):
    li = os.listdir(directory)
    for i in li:
        try:
            print(directory, i)
            b = os.listdir(i)
            show(i + "/" + b)
        except:
            print("file ", i)


# import minibrain
from minibrain import controller,com

# subclass the interface
# the diff drive is auto generated 
# this extends the interface 
class tracks(controller):
    def __init__(self,speed=50000):
        super(tracks,self).__init__(speed)
        self.accel = 0
        self._rate = 100
    
    def rate(self,val):
        self._rate = val 

    def move(self,m1,m2):
        self._send(com.FRAME_RUN,[m1,m2])
    
    def forward(self):
        self.move(self._rate, self._rate)

    def backward(self):
        self.move(-self._rate, -self._rate)

    def left(self):
        self.move(-self._rate, self._rate)

    def right(self):
        self.move(self._rate, -self._rate)

    def bounce(self,count=5,timeout=0.5):
        for i in range(count):
            self.forward()
            time.sleep(timeout)
            self.backward()
            time.sleep(timeout)
        self.move(0,0)
        self.stop()


# Create the main drive objects
d = tracks()

# Some Callback functions for the SPI interface

def mil(a):
    print('millis ->',a)
    return a

def compass(bearing):
    print('bearing :',bearing)
    return bearing 

def current(value):
    print('current -> ',value)
    return current

def outer(a,b,c,d):
    print(" out |",a,b,c,d)
    return (a,b,c,d)

# Bind the callbacks to the drive
#d.bind('getmillis',mil)
#d.bind('compass',compass)
#d.bind('current',current)
d.bind('data',outer)

d.bind('cont',compass)

# Set up the async thread

import uasyncio


def main_runner(reg, app, ws, drive):
    loop = uasyncio.get_event_loop()
    if reg.ws:
        print("Starting Web Socket")
        ws_app = ws.get(drive)
        loop.create_task(ws_app)
    if reg.web:
        print("Starting WebServer")
        app.debug = 0
        import ulogging

        log = ulogging.getLogger("picoweb")
        app.log = log
        app.init()
        loop.create_task(uasyncio.start_server(app._handle, "0.0.0.0", 80))
    if reg.udp:
        print("UDP server starting")
        ud = udpserver.UDPServer(drive)
        loop.create_task(ud.serve(udpserver.cb,'0.0.0.0',12345))
    loop.run_forever()

import _thread
import ws
import udpserver

# Run all the async in this thread
# otherwise bad things happen
print("Starting Async thread")
_thread.start_new_thread(
    main_runner,
    (
        reg,
        web_interface.app,
        ws,
        d,
    ),
)

# Run the telnet server
if reg.telnet:
    print("Starting telnet server")
    import utelnetserver
    utelnetserver.log = True
    utelnetserver.start()

print("Steve Running")

import random

def geti8():
    return random.getrandbits(8)

def check():
    a = geti8()
    b = geti8()
    c = geti8()
    y = geti8()
    (e,f,g,h) = d.data(a,b,c,y)
    if (a == e) and (b == f) and (c == g) and (y == h):
        return True
    else:
        print(a,e)
        print(y,f)
        print(c,g)
        print(d,h)
        return False

def go():
    count = 0 
    while check():
        count += 1
    print("count ",count)
