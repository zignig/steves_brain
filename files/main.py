# Main runner 

# Warning this is auto updated

import picoweb
app = picoweb.WebApp(__name__)

@app.route("/")
def index(req,resp):
    yield from picoweb.start_response(resp)
    htmlFile = open('static/index.html','r')
    #for line in htmlFile:
    #    yield from resp.awrite(line)
    buf = bytearray(32)
    while True:
        l = htmlFile.readinto(buf)
        if not l:
            break
        yield from resp.awrite(buf, 0, l)

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

d = minibrain.diff_drive()


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

import uasyncio

def main_runner(reg,app,ws,mb):
    loop = uasyncio.get_event_loop()
    if reg.ws:
        ws_app = ws.get(mb)
        loop.create_task(ws_app)
    if reg.web:
        app.debug = 0
        import ulogging
        log = ulogging.getLogger("picoweb")
        app.log = log
        app.init()
        loop.create_task(uasyncio.start_server(app._handle,'0.0.0.0',80))
    loop.run_forever()

    

import _thread
import ws
_thread.start_new_thread(main_runner,(reg,app,ws,d,))
if reg.telnet:
     import utelnetserver
     utelnetserver.start()
