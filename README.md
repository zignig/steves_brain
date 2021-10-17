# Upload the firmware
esptool --port /dev/ttyUSB0 -c esp8266 --before no_reset --baud 115200 write_flash  --flash_size=detect 0 esp8266-20210902-v1.17.bin
