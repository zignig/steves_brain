from test import controller 


c = controller()

c.hello()

def stuff(a,b,c):
    print("STUFF",a,b,c)

def stopper():
    print("STOP RIGHT THERE")

def f(data):
    print(data)

c.cb_stop = stopper
c.cb_three = stuff
c.cb_four = f