# change
# This file is executed on every boot (including wake-boot from deepsleep)
# import esp
# esp.osdebug(None)
import uos, machine

# uos.dupterm(None, 1) # disable REPL on UART(0)
import gc
import json
import time
import os 


class Registry:
    def __init__(self,name):
        f = open(name,'r')
        self.name = name 
        data = json.load(f)
        #print(data)
        self._db = data
        f.close()

    def get(self, item):
        if item in self._db:
            val = self._db[item]
            try:
                data = json.loads(val)
            except:
                data = val.decode()
        else:
            data = None
        return data

    def set(self, item, value):
        self._db[item] = json.dumps(value)
        f = open(self.name,'w')
        f.write(json.dumps(self._db))
        f.close()

    def __getattr__(self, item):
        return self.get(item)
    
    def __repr__(self):
        val = ""
        for i in self._db.items():
            val += i[0] + ":" + i[1] + "\n"
        return val


reg = Registry('registry')


# connect to the network
def do_connect():
    # TODO better fallback
    try:
        import network

        # disable ap network esp8266
        # ap = network.WLAN(network.AP_IF)
        # ap.active(False)
        # ap.disconnect()

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
        # wlan = None
        print("Network fail")

    return wlan


wlan = do_connect()

if reg.id is None:
    name = input("name>")
    reg.set("id", name)
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
    import urequests
    path = reg.uplink + "/status/" + reg.id
    data = urequests.get(path).json()
    for i in data:
        local = reg.get("f_" + i)
        remote = data[i]
        # print('>',local,"<>",remote)
        print(i)
        if local != remote:
            if local is None:
                print("local file", i, " missing")
            else:
                print("hash is different")
            print("Fetch file ", i)
            #upip._makedirs("/" + i)
            # os.mkdirs('/'+i)
            path = reg.uplink + "/files/" + reg.id + "/" + i
            print(path)
            r = urequests.get(path)
            f =  open(i,'wb')
            f.write(r.content)
            f.close()
            #upip.save_file(i, upip.url_open(r))
            print("Update registry")
            reg.set("f_" + i, data[i])
            # wait for the flash to catch up
            gc.collect()
            time.sleep(1)


def format_drive():
    print("whoops, that may have been a mistake")
    global reg
    # low level drive format
    print("collecting stuff.")
    b = open("boot.py").read()
    v = ["wifi", "uplink", "ws", "web", "telnet", "id"]
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
    import urequests
    print(time.localtime())
    rtc = machine.RTC()
    path =  reg.uplink + '/time'
    t = urequests.get(path).json()
    val = (t[0], t[1], t[2], 0, t[3], t[4], t[5], 0)
    rtc.datetime(val)
    reg.set("last_timeset", time.localtime())
    return time.localtime()


print("Running Update")
set_time()
update()
gc.collect()
