# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import uos, machine

# uos.dupterm(None, 1) # disable REPL on UART(0)
import gc
import json
import upip

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

    def get(self,item):
        val = self._db.get(item)
        if val is None:
            data = None
        else:
            try:
                data = json.loads(val) 
            except:
                data = val.decode()
        return data 

    def scan(self,prefix):
        for i in self._db.items(prefix+chr(0),prefix+chr(255)):
            print(i)

    def __repr__(self):
        val = ''
        for i in self._db.items():
            val += i[0].decode() + ':' +  i[1].decode() + '\n'
        return val
    
    def __getattr__(self,item):
        return self.get(item)

# Open the registry
reg = Registry()
#reg.list()

# file checkers 
import os

def file_sha(path):
    BLOCK_SIZE = 16
    import os
    import hashlib
    import binascii
    data = bytearray(BLOCK_SIZE)
    stat = os.stat(path)
    file_size = stat[6]
    block_count = file_size // BLOCK_SIZE
    residual = file_size - (block_count * BLOCK_SIZE)
    #print('blocks ',block_count,' | residual ',residual)
    f = open(path,'rb')
    h = hashlib.sha256()
    for i in range(block_count):
        f.readinto(data)
        h.update(data)
    # last partial chunk
    data = f.read(residual)
    h.update(data)
    dig = h.digest()
    #print(dig)
    sha_hex = binascii.hexlify(dig)
    print(path,sha_hex)
    reg.set('f_'+path,sha_hex)
    return sha_hex

class scanner:
    FILE_PREFIX = "f_"
    def __init__(self,path=''):
        self._file_list = []
        self.scan(path)
    
    def scan(self,path):
        for i in os.ilistdir(path):
            file_name = path+'/'+i[0]
            if i[3] != 0:
                print('as file:',file_name)
                self._file_list.append(file_name)
            else:
                print('as folder:',file_name)
                self.scan(file_name)
        
    def update(self):
        for i in self._file_list:
            data = reg.get(scanner.FILE_PREFIX+i)
            if data is None:
                print('file ',i,' missing') 
            hash = file_sha(i)
            reg.set(scanner.FILE_PREFIX+i,hash)

    def __repr__(self):
        st = ''
        for i in self._file_list:
            st = str(i) + '\n'
        return st
        


         
# connect to the network
def do_connect():
    # TODO better fallback
    try:
        import network

        # disable ap network
        #    ap = network.WLAN(network.STA_AP)
        #    ap.active(False)
        #    ap.disconnect()

        wlan = network.WLAN(network.STA_IF)
        wlan.active(True)
        info = reg.wifi
        if info is None:
            nets = wlan.scan()
            for i in nets:
                    print(i[0].decode())
            ssid = input('ssid>')
            password = input('password>')
            reg.set('wifi',[ssid,password])
        if not wlan.isconnected():
            print("connecting to network...")
            wlan.connect(info[0],info[1])
            count = 0
            while not wlan.isconnected():
                #print(wlan.ifconfig())
                count += 1 
                if (count % 10000) == 0:
                    print(wlan.ifconfig()) 
        reg.set('network',wlan.ifconfig())
    except:
        wlan = None
    return wlan


wlan = do_connect()

import socket

def fetch(url,data_type="json",debug=False):
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
    if debug:
        print("HEADERS")
    while True:
      # Receive data
      line= s.readline()
      if line == b'\r\n':
        break 
      val = line.decode().strip().split(':')
      if debug:
        print(val)
      if val[0] == "Content-Length":
        length = int(val[1].strip())    
    if debug:
        print(length)
        print("END HEADERS")
    data = bytearray() 
    while True:
        more = s.recv(length)
        if more == b'':
            break
        if debug:
            print(more)
            print(len(more))
        data.extend(more)
    
    if data_type == "json":
        try:
            data = json.loads(data)
        except Exception as E:
            print(data)
            print(E)
    #print(data)
    # Close the socket    
    s.close()
    return data

try:
    if reg.uplink is None:
        print("enter status url")
        val = input('status>')
        reg.set('uplink',val)
except OSError as e:
    print(e)


def update():
    data = json.load(upip.url_open(reg.uplink+'/status'))
    for i in data:
        local = reg.get('f_'+i)
        remote = data[i]
        print(i)
        if local != remote:
            if local is None:
                print("local file",i," missing")
            else:
                print("hash is different")
            print("Fetch file ",i)
            upip._makedirs(i) 
            upip.save_file(i,upip.url_open(reg.uplink+'/files'+i))
            print("Update registry")
            reg.set('f_'+i,data[i])


print("Running Update")
update()
gc.collect()
