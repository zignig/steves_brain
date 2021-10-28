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

go()
