# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import uos, machine

# uos.dupterm(None, 1) # disable REPL on UART(0)
import gc
import json

gc.collect()

# Data registry class
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
            
    def exists(self,val):
        val = self._db.get(val)
        if val is not  None:
            return True
        else:
            return False

    def set(self,item,data):
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

# Open the registry
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
        nets = wlan.scan()
	for i in nets:
		print(i[0].decode())
        ssid = input('ssid>')
        password = input('password>')
        r.set('wifi',[ssid,password])
        #r._db.put('wifi',json.dumps([ssid,password]))
        #r._db.flush()
    if not wlan.isconnected():
        print("connecting to network...")
        wlan.connect(info[0],info[1])
        count = 0
        while not wlan.isconnected():
            #print(wlan.ifconfig())
            count += 1 
            if (count % 10000) == 0:
                print(wlan.ifconfig()) 
    r.set('network',wlan.ifconfig())
    return wlan


wlan = do_connect()

import socket

def fetch(url,data_type="json"):
    _, _, host, path = url.split('/', 3)
    split_port = host.split(':')
    #print(host,split_port,path)
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
    # status
    d = s.readline()
    #print(d)
    resp = d.split()
    #print(resp)
    #print("HEADERS")
    while True:
      # Receive data
      line= s.readline()
      if line == b'\r\n':
        break 
      val = line.decode().strip().split(':')
    #  print(val)
      if val[0] == "Content-Length":
        length = int(val[1].strip())    
    #print("END HEADERS")
    data = s.recv(length)
    if data_type == "json":
        data = json.loads(data)
    #print(data)
    # Close the socket    
    s.close()
    return data

try:
    files = fetch(r.status)
    print(files)
except OSError as e:
    print(e)


