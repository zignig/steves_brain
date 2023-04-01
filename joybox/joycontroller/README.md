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

# grab eeprom

avrdude -c arduino  -P /dev/serial/by-id/usb-FTDI_FT232R_USB_UART_A700eCR5-if00-port0 -p atmega328p  -b 57600 -U eeprom:r:eeprom.hex:i


# eeprom fail code.

avr-hal-f3855d5807fdfd57/d191b57/avr-hal-generic/src/eeprom.rs#L181