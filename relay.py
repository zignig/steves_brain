" File relay web app"

from flask import Flask
from flask import jsonify
import hashlib 

app = Flask(__name__)
import os,time 
base_path = 'files'

key = "xkDoOyC05K6DeKIr/37beQg8YeA0KnYlF98PYG2W6CQ=\\n"

@app.route('/')
@app.route('/index.html')
def index():
    return 'index'

@app.route('/time')
def get_time():
    # this gets transformed for the RTC onboard.
    # should just be default local time for others.
    t = time.localtime()
    return jsonify(t)
    
# scan the files and get the sha sums of the files   
def scanner(path,data):
    sc = os.scandir(path)
    for i in sc:
        print(i.path)
        if i.is_dir():
            scanner(i.path,data)
        else:
            # chop of the base
            if not path.endswith('.swp'):
                pos = i.path.find('/')
                # get the sha sums
                h = hashlib.sha256()
                h.update(open(i.path,'rb').read())
                r = h.hexdigest()
                data[i.path[pos:]] = r 
    return data

@app.route('/files/<path:path>')
def files(path):
    try:
        p = base_path+'/'+path
        print(p)
        os.stat(p)
        return(open(p,'rb').read())
    except Exception as e :
        print("FAIL",e)
        return str(path)
    

@app.route('/status')
def status():
    data = []
    data = scanner(base_path,{})
    return jsonify(data)

@app.route('/uplink')
def uplink():
    return 'hello'

if __name__ == "__main__":
    app.run(host='0.0.0.0',port=5001,debug=True) 
