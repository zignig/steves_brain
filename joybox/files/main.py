# Main runner

#import uasyncio
#import _thread

import struct 

def reset():
    machine.reset()

led = machine.Pin(0,machine.Pin.OUT)
switch1 = machine.Pin(4,machine.Pin.IN)
switch2 = machine.Pin(5  ,machine.Pin.IN)


def count(val=200):
    for i in range(val):
        js.display(i+1)

def check(val=200):
    start = time.time_ns()
    count(val)
    finish = time.time_ns()
    delta = (finish - start)
    print(delta,val)

def watch_buttons():
    while True:
        print("switch1 - ",switch1.value())
        print("switch2 - ",switch2.value())
        time.sleep_ms(100)
            
## udp server testing.

# Run the telnet server
# def run_telnet():
#     if reg.telnet:
#         print("Starting telnet server")
#         import utelnetserver
#         utelnetserver.start()

# print(reg.id + " Running")
# run_telnet()

import uasyncio , socket

class data:
    def __init__(self,destination):
        self.destination = destination
        self.sock = socket.socket(socket.AF_INET,socket.SOCK_DGRAM)

    def send(self,mess):
        self.sock.sendto(mess,self.destination)

import joycontrol

js = joycontrol.controller(30000)
js.interval = 6
js.hexdisplay(0xcafef00d)

def read(size=8):
    js.ss.off()
    data = js.port.read(size)
    js.ss.on()
    return list(data) 

def hello():
    return "hi" 

def mil(a):
    print('millis ->',a)
    return a

def two(a,b):
    print('two ->',a,b)
    return (a,b)

def outer(a,b,c,d):
    #print(" out |",a,b,c,d)
    return (a,b,c,d)

js.bind('xy',two)
js.bind('outcontrol',outer)
js.bind('getmillis',mil)
js.bind('hello',hello)

import random

def geti8():
    return random.getrandbits(8) - 128

def check():
    a = geti8()
    b = geti8()
    c = geti8()
    d = geti8()
    (e,f,g,h) = js.outcontrol(a,b,c,d)
    if (a == e) and (b == f) and (c == g) and (d == h):
        return True
    else:
        print(a,e)
        print(b,f)
        print(c,g)
        print(d,h)
        return False

def go():
    count = 0 
    while check():
        count += 1
        js.display(count)
    print("count ",count)