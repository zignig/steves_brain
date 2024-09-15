# Async version 


Some commands to get info 

cargo size --release -- -A

.data                         1362  0x800100
.text                        11282       0x0
.bss                           111  0x800652
.note.gnu.avr.deviceinfo        64       0x0
.debug_info                   1524       0x0
.debug_abbrev                 1442       0x0
.debug_line                     26       0x0
.debug_str                     520       0x0
Total                        16331

Look like lots of static strings taking up ram. 
