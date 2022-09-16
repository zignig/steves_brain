// STEVE's minibrain interface
// 
// SPI comms from the ESP32
// communications protocol 

#include <stdint.h>

#include "comms.h"
#include <SPI.h>

// MOTOR STUFF
//#include "L298NMotorService.h"
#include "Robot.h"
using namespace SteveBot;

// The robot
Robot robot;

// spi boot stolen from
// Written by Nick Gammon
// February 2011
// taken from https://gammon.com.au/spi

volatile uint8_t comm ; 
volatile boolean _boop;
boolean debug = false; 

_comms_packet_t the_packet;


void setup (void)
{
  Robot();
  Serial.begin (115200);   // debugging
  // turn on SPI in slave mode
  SPCR |= bit (SPE);
  // have to send on master in, *slave out*
  pinMode(MISO, OUTPUT);
  pinMode(SCK, INPUT_PULLUP);
  pinMode(MOSI, INPUT);
  pinMode(SS,INPUT_PULLUP);
  // get ready for an interrupt 
  // now turn on interrupts
  SPI.attachInterrupt();
  Serial.println("minibrain 0.1");
}  // end of setup


// SPI interrupt routine
ISR (SPI_STC_vect)
{
  uint8_t c = SPDR;  // grab byte from SPI Data Register
  comms_input_byte(c);
  //_boop = true;
  //comm = SPDR;
}  // end of interrupt routine SPI_STC_vect

// main loop - wait for flag set in interrupt routine
void loop (void)
{
  //if(_boop){
  //  Serial.println(comm);
  //  _boop = false;
  // }
  if(comms_packet_ready()){
    comms_get_packet(&the_packet);
    //Serial.println(comms_packet_ready());
    if(debug==true){
        Serial.println("START FRAME");
        Serial.println(the_packet.type);
        Serial.println(the_packet.checksum);
        Serial.println(the_packet.data1);
        Serial.println(the_packet.data2);
        Serial.println(the_packet.data3);
        Serial.println(the_packet.data4);
        Serial.println("END FRAME");
    }
    comms_packet_ack();
    int lspeed = 0;
    int rspeed = 0;
    // convert to coords 
    if(the_packet.data3 == 1){
        lspeed = -the_packet.data1;
    }else{
        lspeed = the_packet.data1;
    }
    if(the_packet.data4 == 1){
        rspeed = -the_packet.data2;
    }else{
        rspeed = the_packet.data2;
    }
    int val1 = the_packet.data1;
    switch(the_packet.type){
        case FRAME_HELLO:
            Serial.println("hello");
            break;
        case FRAME_STOP:
            Serial.println("stop");
            break;
        case FRAME_RUN:
            Serial.println("run");
            // Set the motor speed
            robot.SetDiff(lspeed,rspeed);
            break;
        case FRAME_SETACC:
            robot.SetAcceleration(val1);
            break;
        case FRAME_SETJOY:
            robot.SetJoy(lspeed,rspeed);
            break;
        case FRAME_SETTIMEOUT:
            robot.SetTimeout(val1 << 3);
            break;
    }
  }
  robot.Update();
}  // end of loop
