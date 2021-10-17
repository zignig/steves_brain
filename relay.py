" File relay web app"

from flask import Flask
from flask import jsonify

app = Flask(__name__)
import os 
base_path = './files'

def scanner(path,data):
    sc = os.scandir(path)
    for i in sc:
        print(i.path)
        if i.is_dir():
            scanner(i.path,data)
        else:
            # chop of the base
            pos = i.path.find('/')
            data[i.path[pos:]] = 'sha'
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

if __name__ == "__main__":
    app.run(host='0.0.0.0',port=5001) 
