# Todo for the joy box

in no particular order.

- Arduino
  - scale analogue controller with calibration to i8
  - return an option for the joystick on change.
  - have invert flag for the axes.
  - fix comms to unroll commands of the ring buffer 
  - return default frame when empty
  - code for buttons , generate events.
    - press release for switches and buttons
    - probably need to debounce
  - power supply for the box ( 4 x AA )


- ESP
  - write the udp server and client for bot comms.
- pythonator 
  


## Done
- read the frame size and the leading constants.
- update the code to move frames back to the esp
- swap missile and emergency stop , wiper clashes.
- fix the display overflow issue
- missile switch
- emergency stop 
- left and right buttons
- top switches and top led
  - check wiring
  - soft interface on esp
- double check the dimensions 
- update the dxf for the box creation
- cut the box carefully and mount the electronics 


# Code cleanup
- Break out the comms system
  - have a generic frame
- set serial_println! into a normal mutex rather than critical section
- update the rover to the new comms and rebuild the control
- update the boot.py
  - query for the id
  - save the unique id 
  - scan and save multiple wifi networks
  - active the AP when there are no wifi networks for a period of time.

 
# Long
- rewrite ampy and esptool in rust ( large ) 
- rewrite the relay in rust
- make the console ( split the uart)