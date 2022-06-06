# Main runner 

# Warning this is auto updated

import picoweb
app = picoweb.WebApp(__name__)

@app.route("/")
def index(req,resp):
    yield from picoweb.start_response(resp)
    htmlFile = open('static/index.html','r')
    for line in htmlFile:
        yield from resp.awrite(line)

@app.route('/status')
def status(req,resp):
    yield from picoweb.start_response(resp)
    yield from resp.awrite('status OK')

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

import minibrain 

d = minibrain.diff_drive(10000)

# TODO , convert to spi
# connection
# green , pin 12 , MISO
# yellow , pin 13 , MOSI
# white , pin 14 , CLK
# blue , pin 27 , SS

def loopback_test(size=8,sleep=40):
    import mpyaes
    import time
    # empty the buffer
    print(d.port.read())
    print(d.port.read())
    print(d.port.read())
    for i in range(size):
        data = mpyaes.generate_key(16)
        length = d.port.write(data)
        time.sleep_ms(sleep)
        recv = d.port.read(length)
        if recv != None:
            recv = bytearray(recv)
        if data == recv:
            print(i,'ok')
        else:
            print(i,'fail :',data,recv)


import _thread
if reg.web:
    _thread.start_new_thread(go,())
if reg.telnet:
    
    import utelnetserver
    utelnetserver.start()
if reg.ws:
    import ws
    _thread.start_new_thread(ws.go,())
