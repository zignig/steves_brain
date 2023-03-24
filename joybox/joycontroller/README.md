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

9ec674c04d4ca3fd0b9157a3d1e985e48355b9677109eb4155419ee9d80b0cc51f701df60f89a2911da118df8b1d808e0d0a462b579107dde734cef193db82f4  text.hex