// interface for the buttons and switches. 

// buttons , switches
// debounce , state and events.
use arduino_hal;
use arduino_hal::port::{mode, Pin, PinOps};

pub struct Button<P1> { 
    pin: Pin<mode::Input<mode::Floating>,P1>,
    val : bool
}

impl<P1:PinOps> Button<P1> {
    pub fn new(pin: Pin<mode::Input<mode::Floating>,P1>) -> Self{
        Self {
            pin: pin,
            val: false
        }
    }

    pub fn read(&mut self)-> bool{
        if self.pin.is_low(){
            self.val = false;
            return false
        } else {
            self.val = true;
            return true
        }
    }
}


pub struct Buttons<B1,B2,B3,B4> { 
    vals: (Button<B1>,Button<B2>,Button<B3>,Button<B4>),
}

impl <B1:PinOps,B2:PinOps,B3:PinOps,B4:PinOps>Buttons <B1,B2,B3,B4>{ 
    pub fn new(buttons:(Button<B1>,Button<B2>,Button<B3>,Button<B4>) ) -> Self { 
        Self { vals: buttons }
    }

    // pub fn read(&mut self) -> u8{
    //     let mut value: u8 = 0;
    //     let mut bit: u8 = 1;
    //     for pos in 0..3{
    //         if self.vals[pos].read() { 
    //             value = value | ((1 << pos) as u8);
    //         }
    //     }
    //     value
    // }
}