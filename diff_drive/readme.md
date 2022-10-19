# Stuff to do

[x] get the compass connected
[x] add the current sensor
[x] min and max on the current sensor 

- add the distance sensor ( in progress )
- have spi comms return data to the esp32
    - need to rework the frames , possibly 8 byte payload
    - status
    - current motor status 
    - current sensor
    - distance sensor
    - compass heading

- have spi be able to read a frame after a send ( sensor info back to esp)

- expose the timeout as a command 

- expose trigger and minspeed as a command

- servo driver , need to move one o the Diffdrives pins for PWM

- elaborate the web interface so it has more than one page.

- rework the commands from the web client to be better intervals.


- web socket back to the relay for data collection.

- thread locks for inter comms on the esp32 
## Cortex

the cortex needs to sit between the input and the drive
Add a small mutable NN inbetween commands and actions. Sensor 
data needs to be able to overide user input.

- save and load the data from eeprom

