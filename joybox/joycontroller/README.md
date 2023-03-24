# Joystick Controller

- 3 Axis joystick 
- Throttle
- 8 digit display
- 2 buttons 
- 2 switches
- 2 leds

## Compute 

- ESP8266
- 5 -> 3.3 bridge 
- Arduino pro mini

A general robotics controller , made for the steve bot.

# Run as simulation 

simavr  -m atmega328p -f 16000000 target/avr-atmega328p/debug/joycontroller.elf

avr-objcopy -O ihex  target/avr-atmega328p/debug/joycontroller.elf  data.hex
