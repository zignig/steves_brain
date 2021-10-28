
"""Simple demo of using Flask with aiohttp via aiohttp-wsgi's
WSGIHandler.
"""

import asyncio
import aiohttp
from aiohttp import web
from aiohttp_wsgi import WSGIHandler
from flask import Flask, render_template

app = Flask('aioflask')
app.config['DEBUG'] = True
app.jinja_loader.searchpath.insert(0, '.')


def counter():
    num = 0
    while True:
        yield num
        num += 1
        if num == 10:
            return


@app.route('/')
def index():
    return render_template('index.html')


async def socket(request):
    ws = web.WebSocketResponse()
    await ws.prepare(request)
    async for msg in ws:
        print(msg.data)
        if msg.type == aiohttp.WSMsgType.TEXT:
            print(msg.data)
            if msg.data == 'close':
                await ws.close()
            else:
                await ws.send_str(msg.data + '/answer')
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
    web.run_app(aio_app, port=5002)
