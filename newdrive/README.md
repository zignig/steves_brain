# Minibrain for steve ( the robot)

A promini arduino to drive motors and compass and a current senspr

Re-written in rust after c++ 

# BUGS !!

There appears to be a problem with the interrupts ! , AVR does not have nested interupts !!

as per (this issue)[https://github.com/Rahix/avr-hal/issues/75] it turns out that you need to minimize the amount of code that runs inside an interrupt. I have found that losing ticks with the string print and the SPI interrupt. 

The system needs to have the _absolute_ minimum, the comms.rs needs to be stripped to just add the byte into the buffer and then set a flag. Then the main loop can take process the packet.

Having the string printing use an interrupt is probably a fail. Need to look into general mutex.

If there is too much string printing , it will miss a tick. 

---


