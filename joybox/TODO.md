# Todo for the joy box

in no particular order.

- swap missile and emergency stop , wiper clashes.
- top switches and top leds. ( one switch broken , esp pinout...)
- update the code to move frames back to the esp
- write the udp server and client for bot comms.
- update the dxf for the box creation (ongoing)
- double check the dimensions 
- cut the box carefully and mount the electronics 
- power supply for the box ( 4 x AA )
## Done
 - fix the display overflow issue
 - missle switch
 - emergency stop 
 - left and right buttons

# Code cleanup
- Break out the comms system 
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