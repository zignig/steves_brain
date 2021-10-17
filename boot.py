# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import uos, machine

# uos.dupterm(None, 1) # disable REPL on UART(0)
import gc
import json

gc.collect()

class Registry:
    def __init__(self):
        # create the registry 
        import btree
        try:
            f = open('registry','r+b')
        except:
            f = open('registry','w+b')
        self._db = btree.open(f)

    def list(self):
        for i in self._db.items():print(i)
            
    def set(self,item,data):
        print(self,item,data)
        self._db[item] = json.dumps(data)
        self._db.flush()

    def __getattr__(self,item):
        val = self._db.get(item)
        if val is None:
            data = None
        else:
            try:
                data = json.loads(val) 
            except:
                data = val.decode()
        return data 

r = Registry()
r.list()

# connect to the network
def do_connect():
    import network

    # disable ap network
    #    ap = network.WLAN(network.STA_AP)
    #    ap.active(False)
    #    ap.disconnect()

    wlan = network.WLAN(network.STA_IF)
    wlan.active(True)
    info = r.wifi
    if info is None:
        print(wlan.scan())
        ssid = input('ssid>')
        password = input('password>')
        r._db.put('wifi',json.dumps([ssid,password]))
        r._db.flush()
    if not wlan.isconnected():
        print("connecting to network...")
        wlan.connect(info[0],info[1])
        while not wlan.isconnected():
            pass
    print("network config:", wlan.ifconfig())
    return wlan


wlan = do_connect()

import socket

def fetch(url):
    _, _, host, path = url.split('/', 3)
    split_port = host.split(':')
    print(host,split_port,path)
    if len(split_port) > 1:
        port = int(split_port[1])
        host = split_port[0]
    else:
        port = 80
    addr = socket.getaddrinfo(host,port)[0][-1]
     
    # Create a socket
    s = socket.socket()
    # Connect to IP address
    s.connect(addr)
    # Send GET request
    s.send(bytes('GET /%s HTTP/1.0\r\nHost: %s\r\n\r\n' % (path, host), 'utf8'))
    full = b'' 
    # status
    d = s.readline()
    print(d)
    resp = d.split()
    print(resp)
    print("HEADERS")
    while True:
      # Receive data
      line= s.readline()
      if line == b'\r\n':
        break 
      val = line.decode().strip().split(':')
      print(val)
      #if data:
      #  print(str(data), end='')
      #  full += data
      #else:
      #  break
    # Close the socket    
    print("END HEADERS")
    data = s.recv(1024)
    print(data)
    s.close()
    #return str(full)

fetch('http://noid.erf:5001/status')

import picoweb

app = picoweb.WebApp(__name__)

@app.route("/")
def index(req,resp):
    yield from picoweb.start_response(resp)
    htmlFile = open('static/index.html','r')
    for line in htmlFile:
        yield from resp.awrite(line)

def go():
    print("running web service")
    app.run(host='0.0.0.0',port=80,debug=True)


import os
def show(directory='/'):
    li = os.listdir(directory)
    for i in li:
        try:
            print(directory,i)
            b = os.listdir(i)
            show(i+'/'+b)
        except:
            print('file ',i)

go()
