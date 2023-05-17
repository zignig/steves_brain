# Todo for the joy box

in no particular order.

- Arduino


  - move config down and have a status check on low eeprom
    - if empty , load defaults.
    - query callibrate.
  - idle scanner for display 
  
    - Jensen scanner on decimal point
  - appease clippy.
    - ve is so rude.
  - return an option for the joystick on change.
    - keep last value , check and Option forward
  - fix comms to unroll commands of the ring buffer 
  - code for buttons , generate events.
    - press release for switches and buttons
    - probably need to debounce

- ESP
  - startup
    - check millis , see elapsed time.
  - idle state
    - if nothing is happening on the joy stick or buttons.
      - don't send packet
      - 
  - write the udp server and client for bot comms.
  - convert to single uasync 
    - query

- pythonator 
  - if binding does not match return list.
  
- phys
  - hot glue coat the M3 cap head for contact.
  - clean edges and polish. 
  - power supply for the box ( 4 x AA )

## Done
- Joycontroller
  - have invert flag for the axes.
  - return default frame when empty
  - scale analogue controller with calibration to i8
  - read the frame size and the leading constants.
  - update the code to move frames back to the esp
  - swap missile and emergency stop , wiper clashes.
  - fix the display overflow issue
- JoyBox ( the hardware )
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