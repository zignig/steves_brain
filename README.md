# Steves Brain

This project is the source code for steves brain.

https://zignig.github.io/tags/steve/ has some outdated entries on the build. I am currently writing an updated entry.

## Getting started.

<hr>

# Upload the firmware
## ESP8266


python2 /opt/esp8266/esp-open-sdk/esptool/esptool.py  --port /dev/ttyUS0 erase_flash



esptool --port /dev/serial/by-id/usb-1a86_USB_Serial-if00-port0  --baud 460800 write_flash --flash_size=detect 0  esp8266-20220618-v1.19.1.bin 

## ESP32

esptool --chip esp32 --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x1000 esp32-20190125-v1.10.bin

## FORMAT

import os
import flashbdev
os.VfsLfs2.mkfs(flashbdev.bdev)

## COOL BUTTON

https://codepen.io/jemware/pen/ojhCp

## Interface from 

https://www.instructables.com/Wifi-PPM-no-App-Needed/

modified.

# SPI trouble

## BROKEN CLOCK

so ... after getting spi working on two usb feeds.

when it gets moved to (battery) only 

## 20220612 upadte 

-  replaced with fixed level shifters, spi working 
-  power brownout with heavy motor usage, need to rework regulation

## 20200708 update

- spi fixed , asyncio weirdness fixed
- don't run asyncio in two threads...

