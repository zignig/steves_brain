// frame based comms protocol
// sync1 , sync2 , action , checksum , data1,data2,data3,data4
// all bytes 
#ifndef INC_COMMS_H_
#define INC_COMMS_H_

#include <stdint.h>
#include <stdbool.h>

#define SYNC_1 0xF
#define SYNC_2 0xE

typedef struct _comms_packet_t {
    uint8_t sync1;
    uint8_t sync2;
    uint8_t type;
    uint8_t checksum;
    uint8_t data1;
    uint8_t data2;
    uint8_t data3;
    uint8_t data4;
} comms_packet_t;

typedef enum _comms_type_e {
    FRAME_HELLO = 0,
    FRAME_STOP,
    FRAME_RUN,
    FRAME_SETACC,
    FRAME_SETJOY,
    FRAME_SETTIMEOUT,
    FRAME_SETTRIGGER,
    FRAME_SETMINSPEED,
    FRAME_SENSOR,
    FRAME_CONFIG,
    FRAME_COUNT
} comms_type_e;

typedef union _packet_builder_t {
    uint8_t buff[8];
    comms_packet_t packet;
} packet_builder_t;

bool comms_input_byte(uint8_t byte);
bool comms_packet_valid(comms_packet_t* packet);
void comms_input_packet(uint8_t* packet);
bool comms_get_packet(comms_packet_t* packet);
void comms_build_packet(comms_packet_t* packet, comms_type_e type, uint8_t* data);
bool comms_packet_ready();
void comms_packet_ack();

#endif
