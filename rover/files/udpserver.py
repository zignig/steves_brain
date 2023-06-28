import uselect
import usocket
import uasyncio
import json

log = False

# UDP server
class UDPServer:
    def __init__(self, drive, polltimeout=1, max_packet=1024):
        self.polltimeout = polltimeout
        self.max_packet = max_packet
        self.drive = drive

    def close(self):
        self.sock.close()

    async def serve(self, cb, host, port, backlog=5):
        ai = usocket.getaddrinfo(host, port)[0]  # blocking!
        s = usocket.socket(usocket.AF_INET, usocket.SOCK_DGRAM)
        self.sock = s
        s.setblocking(False)
        s.bind(ai[-1])

        p = uselect.poll()
        p.register(s, uselect.POLLIN)
        to = self.polltimeout
        while True:
            try:
                if p.poll(to):
                    buf, addr = s.recvfrom(self.max_packet)
                    ret = cb(buf, addr,self.drive)
                    await uasyncio.sleep(0)
                    if ret:
                        s.sendto(ret, addr)  # blocking
                    del buf, addr
                await uasyncio.sleep(0)
            except uasyncio.core.CancelledError:
                # Shutdown server
                s.close()


port = 12345


def cb(msg, adr,drive):
    #print("Got:", msg)#, " from ", adr)
    #print(drive)
    data = json.loads(msg)
    if log:
        print(data)
    if (data[0] != 0) | (data[1] != 0):
        drive.setjoy(data[0],data[1])
    return "ack".encode("ascii")


def go():
    s = UDPServer()
    l = uasyncio.get_event_loop()
    l.run_until_complete(s.serve(cb, "0.0.0.0", port))
