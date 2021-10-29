# differential steering
# from http://github.com/edumardo/DifferentialSteering

class DiffDrive:
    RANGE = 127
    def __init__(self,piv_limit=10):
        self.lmotor = 0
        self.rmotor = 0

        self.piv_limit = piv_limit
        self.piv_speed = 0
        self.piv_scale = 0 

        self.lpremix = 0
        self.rpremix = 0

        self.rate = 0.1
        
        self.range = 0



    def calc(self,xval,yval):
        if yval >= 0:
            # Forward
            if xval >= 0:
                self.lpremix = self.RANGE
                self.rpremix = self.RANGE - xval
            else:
                self.lpremix = self.RANGE + xval
                self.rpremix = self.RANGE
        else:
            # Reverse
            if xval >= 0:
                self.lpremix = self.RANGE - xval
                self.rpremix = self.RANGE
            else:
                self.lpremix = self.RANGE   
                self.rpremix = self.RANGE + xval

        # Rescale
        self.lpremix = self.lpremix * yval / self.RANGE
        self.rpremix = self.rpremix * yval / self.RANGE

        # Calc Pivot
        self.pivot_speed = xval
        if abs(yval) > self.piv_limit:
            self.piv_scale = 0
        else:
            self.piv_scale = (1.0 - abs(yval)) / self.piv_limit


        # Calculate the mix
        self.lmotor = ( 1.0 - self.piv_scale ) * self.lpremix + self.piv_scale * self.piv_speed
        self.rmotor = ( 1.0 - self.piv_scale ) + self.rpremix + self.piv_scale * (-self.piv_speed)




