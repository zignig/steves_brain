
import picoweb

app = picoweb.WebApp(__name__)


def stream_file(resp,file_name, content='text/html'):
    yield from picoweb.start_response(resp,content_type=content)
    htmlFile = open(file_name, "r")
    # for line in htmlFile:
    #    yield from resp.awrite(line)
    buf = bytearray(32)
    while True:
        l = htmlFile.readinto(buf)
        if not l:
            break
        yield from resp.awrite(buf, 0, l)
    htmlFile.close()

# the default landing page
@app.route("/")
def index(req, resp):
    yield from stream_file(resp,'static/index.html')

@app.route("/controller")
def controller(req,resp):
    yield from stream_file(resp,'static/controller.html')

@app.route("/app.css")
def css(req,resp):
    yield from stream_file(resp,'static/app.css',content="text/css")

@app.route("/status")
def status(req, resp):
    yield from picoweb.start_response(resp)
    yield from resp.awrite("status OK")


def go():
    print("running web service")
    app.run(host="0.0.0.0", port=80, debug=True)