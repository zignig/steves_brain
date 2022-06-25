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
using namespace Taibot;

Robot robot;

// Left Motor (A)
int Lenable = 3;
int L1 = 9;
int L2 = 8;
// Right Motor (B)
int Renable = 5;
int R2 = 7;
int R1 = 6;


//L298NMotorService leftMotor(true,true,Lenable,L1,L2);
//L298NMotorService rightMotor(true,true,Renable,R1,R2);

// spi boot stolen from
// Written by Nick Gammon
// February 2011
// taken from https://gammon.com.au/spi


volatile uint8_t comm ; 
volatile boolean _boop;

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
  //leftMotor.Setup();
  //rightMotor.Setup();
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
    Serial.println("START FRAME");
    Serial.println(the_packet.type);
    Serial.println(the_packet.checksum);
    Serial.println(the_packet.data1);
    Serial.println(the_packet.data2);
    Serial.println(the_packet.data3);
    Serial.println(the_packet.data4);
    Serial.println("END FRAME");
    comms_packet_ack();
    int lspeed = 0;
    int rspeed = 0;
    switch(the_packet.type){
        case COMMS_TYPE_HELLO:
            Serial.println("hello");
            break;
        case COMMS_TYPE_STOP:
            Serial.println("stop");
            break;
        case COMMS_TYPE_RUN:
            Serial.println("run");
            // Deal with directions
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
            // Set the motor speed
            //leftMotor.SetSpeed(lspeed); 
            //rightMotor.SetSpeed(rspeed); 
            robot.SetDiff(lspeed,rspeed);
            break;
        case COMMS_TYPE_SETACC:
            Serial.println("acc");
            int acc = the_packet.data1;
            robot.SetAcceleration(acc);
            break;
    }
  }
  robot.Update();
  //leftMotor.Update();
  //rightMotor.Update();
}  // end of loop
