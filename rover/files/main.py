# Main runner

# Warning this is auto updated
import web_interface



import os

def show(directory="/"):
    li = os.listdir(directory)
    for i in li:
        try:
            print(directory, i)
            b = os.listdir(i)
            show(i + "/" + b)
        except:
            print("file ", i)


import minibrain

d = minibrain.diff_drive()


import uasyncio


def main_runner(reg, app, ws, mb):
    loop = uasyncio.get_event_loop()
    if reg.ws:
        print("Starting Web Socket")
        ws_app = ws.get(mb)
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
        ud = udpserver.UDPServer()
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
    utelnetserver.start()

print("Steve Running")