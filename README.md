# Upload the firmware

## ESP8266

esptool --port /dev/ttyUSB0 -c esp8266 --before no_reset --baud 115200 write_flash  --flash_size=detect 0 esp8266-20210902-v1.17.bin

## ESP32

esptool --chip esp32 --port /dev/ttyUSB0 --baud 460800 write_flash -z 0x1000 esp32-20190125-v1.10.bin

## FORMAT

import os
import flashbdev
os.VfsFat.mkfs(flashbdev.bdev)
