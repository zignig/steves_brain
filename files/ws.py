from websockets.client import connect
from websockets.server import serve
import uasyncio as asyncio
import struct, time


class WS_SERVER:
    def __init__(self,control):
        self.control = control 
        self._lastUpdate = time.ticks_us()

    async def client_test(self):
        ws = await connect("ws://192.168.3.19:7777/test")
        if not ws:
            print("connection failed")
            return
        await ws.send("[10,10]")
        print(await ws.recv())
        await ws.wait_closed()


    async def add_client(self,ws, path):
        print("Connection on {}".format(path))
        x = 0
        y = 0
        try:
            async for msg in ws:
                data = struct.unpack('bb',msg)
                #print(data)
                if data[0] == 2:
                    x = -data[1]
                if data[0] == 3:
                    y = data[1]
                now = time.ticks_us()
                if( (self._lastUpdate + self.control.interval ) < now):
                    print(x,y)                
                    self.control.joy(x*2,y*2)
                    self._lastUpdate = now
                #await ws.send(msg)
        finally:
            print("Disconnected")

def go(control):   
    wss = WS_SERVER(control)
    ws_server = serve(wss.add_client, "0.0.0.0", 7777)
    loop = asyncio.get_event_loop()
    loop.run_until_complete(ws_server)
    loop.run_forever()
