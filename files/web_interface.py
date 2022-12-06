
import picoweb

app = picoweb.WebApp(__name__)


# the default landing page
@app.route("/")
def index(req, resp):
    yield from picoweb.start_response(resp)
    htmlFile = open("static/index.html", "r")
    # for line in htmlFile:
    #    yield from resp.awrite(line)
    buf = bytearray(32)
    while True:
        l = htmlFile.readinto(buf)
        if not l:
            break
        yield from resp.awrite(buf, 0, l)
    htmlFile.close()


@app.route("/status")
def status(req, resp):
    yield from picoweb.start_response(resp)
    yield from resp.awrite("status OK")


def go():
    print("running web service")
    app.run(host="0.0.0.0", port=80, debug=True)