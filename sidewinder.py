"""Simple demo of using Flask with aiohttp via aiohttp-wsgi's
WSGIHandler.
"""

import asyncio
import aiohttp
from aiohttp import web
from aiohttp_wsgi import WSGIHandler
from flask import Flask, render_template
from diff import DiffDrive
import json

app = Flask('steve_controller')
app.config['DEBUG'] = True
app.jinja_loader.searchpath.insert(0, '.')

dd =DiffDrive()


@app.route('/')
def index():
    print('index')
    return render_template('index.html')

@app.route('/static/<path:path>')
def serve_static(path):
    return str(path)
    return app.send_static_file(path)

commands = {
        0 : 'status',
        4 : 'button1',
        5 : 'button2',
        6 : 'button3',
        7 : 'button4',
        }

drive = [0,0]
trim = [0,0]

async def socket(request):
    ws = web.WebSocketResponse()
    await ws.prepare(request)
    async for msg in ws:
        if msg.type == aiohttp.WSMsgType.BINARY:
            data = msg.data
            source = data[0]
            value = (data[1] << 8 )+ data[2]
            if source in commands:
                print(commands[source],value)
            if source == 0: 
                trim[0] = value - 128
            if source == 1: 
                trim[1] = value - 128
            if source == 2: 
                drive[0] = value - 128
            if source == 3: 
                drive[1] = value - 128
            print(drive)
            dd.calc(drive[0],drive[1])
            print(dd.lmotor,dd.rmotor)
        if msg.type == aiohttp.WSMsgType.TEXT:
            print(msg.data)
            if msg.data == 'close':
                await ws.close()
            else:
                await ws.send_str(json.dumps(55))
        elif msg.type == aiohttp.WSMsgType.ERROR:
            print('ws connection closed with exception %s' %
                  ws.exception())

    print('websocket connection closed')

    return ws


if __name__ == "__main__":
    loop = asyncio.get_event_loop()
    aio_app = web.Application()
    wsgi = WSGIHandler(app)
    aio_app.router.add_route('*', '/{path_info: *}', wsgi.handle_request)
    aio_app.router.add_route('GET', '/socket', socket)
    web.run_app(aio_app, port=5555)
