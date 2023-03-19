# change
# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import uos, machine

# uos.dupterm(None, 1) # disable REPL on UART(0)
import gc
import json
import upip
import time

gc.collect()

# Data registry class
class Registry:
    "Wrapper around the btree construct"

    def __init__(self):
        # create the registry
        import btree

        try:
            f = open("registry", "r+b")
        except:
            f = open("registry", "w+b")
        self._db = btree.open(f)

    def list(self):
        for i in self._db.items():
            print(i)

    def exists(self, val):
        val = self._db.get(val)
        if val is not None:
            return True
        else:
            return False

    def set(self, item, data):
        self._db[item] = json.dumps(data)
        self._db.flush()

    def get(self, item):
        val = self._db.get(item)
        if val is None:
            data = None
        else:
            try:
                data = json.loads(val)
            except:
                data = val.decode()
        return data

    def scan(self, prefix):
        for i in self._db.items(prefix + chr(0), prefix + chr(255)):
            print(i)

    def __repr__(self):
        val = ""
        for i in self._db.items():
            val += i[0].decode() + ":" + i[1].decode() + "\n"
        return val

    def __getattr__(self, item):
        return self.get(item)


# Open the registry
reg = Registry()
# reg.list()

# file checkers
import os


def file_sha(path):
    BLOCK_SIZE = 16
    import os
    import hashlib
    import binascii

    data = bytearray(BLOCK_SIZE)
    stat = os.stat(path)
    file_size = stat[6]
    block_count = file_size // BLOCK_SIZE
    residual = file_size - (block_count * BLOCK_SIZE)
    # print('blocks ',block_count,' | residual ',residual)
    f = open(path, "rb")
    h = hashlib.sha256()
    for i in range(block_count):
        f.readinto(data)
        h.update(data)
    # last partial chunk
    data = f.read(residual)
    h.update(data)
    dig = h.digest()
    # print(dig)
    sha_hex = binascii.hexlify(dig)
    print(path, sha_hex)
    reg.set("f_" + path, sha_hex)
    return sha_hex


class scanner:
    FILE_PREFIX = "f_"

    def __init__(self, path=""):
        self._file_list = []
        self.scan(path)

    def scan(self, path):
        for i in os.ilistdir(path):
            file_name = path + "/" + i[0]
            if i[3] != 0:
                print("as file:", file_name)
                self._file_list.append(file_name)
            else:
                print("as folder:", file_name)
                self.scan(file_name)

    def update(self):
        for i in self._file_list:
            data = reg.get(scanner.FILE_PREFIX + i)
            if data is None:
                print("file ", i, " missing")
            hash = file_sha(i)
            reg.set(scanner.FILE_PREFIX + i, hash)

    def __repr__(self):
        st = ""
        for i in self._file_list:
            st = str(i) + "\n"
        return st


# connect to the network
def do_connect():
    # TODO better fallback
    try:
        import network

        # disable ap network esp8266
        #ap = network.WLAN(network.AP_IF)
        #ap.active(False)
        #ap.disconnect()

        wlan = network.WLAN(network.STA_IF)
        wlan.active(True)
        info = reg.wifi
        if info is None:
            nets = wlan.scan()
            for i in nets:
                print(i[0].decode())
            ssid = input("ssid>")
            password = input("password>")
            reg.set("wifi", [ssid, password])
        outer = 0
        if not wlan.isconnected():
            print("connecting to network...")
            wlan.connect(info[0], info[1])
            count = 0
            while not wlan.isconnected():
                # print(wlan.ifconfig())
                count += 1
                if (count % 100000) == 0:
                    print(wlan.ifconfig())
                    outer += 1
                    if outer == 60:
                        break
        reg.set("network", wlan.ifconfig())
        print(reg.network)
    except:
        wlan = None
        print("Network fail")

    return wlan


wlan = do_connect()

try:
    if reg.uplink is None:
        print("enter status url")
        val = input("status>")
        reg.set("uplink", val)
        reg.set("telnet", True)
except OSError as e:
    print(e)


def update():
    "Get the updates"
    import upip
    data = json.load(upip.url_open(reg.uplink + "/status/"+reg.id))
    for i in data:
        local = reg.get("f_" + i)
        remote = data[i]
        print(i)
        if local != remote:
            if local is None:
                print("local file", i, " missing")
            else:
                print("hash is different")
            print("Fetch file ", i)
            #upip._makedirs(i)
            upip.save_file(i, upip.url_open(reg.uplink + '/files/' + reg.id +'/'+ i))
            print("Update registry")
            reg.set("f_" + i, data[i])
            # wait for the flash to catch up
            gc.collect()
            time.sleep(2)


def format_drive():
    print("whoops, that may have been a mistake")
    global reg
    # low level drive format
    print("collecting stuff.")
    b = open("boot.py").read()
    v = ["wifi", "uplink", "ws", "web", "telnet","id"]
    d = {}
    for i in v:
        d[i] = reg.get(i)
    # format the drive
    reg._db.close()
    del reg
    import os
    import flashbdev

    os.VfsLfs2.mkfs(flashbdev.bdev)
    print("too late now...")
    # write the boot back down
    f = open("boot.py", "w")
    f.write(b)
    f.close()
    rnew = Registry()
    for i in d:
        rnew.set(i, d[i])
    rnew._db.flush()
    rnew._db.close()
    print("all gone, rebuild...")
    machine.reset()


def set_time():
    print(time.localtime())
    rtc = machine.RTC()
    t = json.load(upip.url_open(reg.uplink + "/time"))
    # esp32 rtc has a weird format
    # weekday in the middle
    val = (t[0],t[1],t[2],0,t[3],t[4],t[5],0)
    rtc.datetime(val)
    reg.set('last_timeset',time.localtime())
    return time.localtime()


print("Running Update")
set_time()
update()
gc.collect()
