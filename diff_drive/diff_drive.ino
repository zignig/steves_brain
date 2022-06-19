// STEVE's minibrain interface
// 
// SPI comms from the ESP32
// communications protocol 

#include <stdint.h>
#include "settings.h"
#include "drive.h"

#include "comms.h"
#include <SPI.h>

// Left Motor (A)
int Lenable = 3;
int L1 = 9;
int L2 = 8;
// Right Motor (B)
int Renable = 5;
int R1 = 7;
int R2 = 6;
// speed and time length
int sp = 100;
int len = 500;

int inByte = 0 ;
int counter  = 0;



void enable()
{
  pinMode(Lenable, OUTPUT);
  pinMode(Renable, OUTPUT);
  pinMode(L1, OUTPUT);
  pinMode(L2, OUTPUT);
  pinMode(R1, OUTPUT);
  pinMode(R2, OUTPUT);
 
}

void disable()
{
  pinMode(Lenable, INPUT);
  pinMode(Renable, INPUT);
  pinMode(L1, INPUT);
  pinMode(L2, INPUT);
  pinMode(R1, INPUT);
  pinMode(R2, INPUT);
 
}

void moveBot(bool dir, int spd, int dur) {
  // Motor A
  digitalWrite(L1, dir);
  digitalWrite(L2, !dir);  //The '!' symbol inverts the boolean value. So for example, if dir is true, !dir is false.
  // Motor B
  digitalWrite(R1, !dir);
  digitalWrite(R2, dir);
  // Set motor speed to spd
  analogWrite(Lenable, spd);
  analogWrite(Renable, spd);
  //Motion Duration
  delay(dur);
}

void rotateBot(bool dir, int spd, int dur) {
  // Motor A
  digitalWrite(L1, !dir);
  digitalWrite(L2, dir); 
  // Motor B
  digitalWrite(R1, !dir);
  digitalWrite(R2, dir);
  // Set motor speed to spd
  analogWrite(Lenable, spd);
  analogWrite(Renable, spd);
  //Rotation Duration
  delay(dur);
}

//Turn off both motors
void stopMotors() {
  digitalWrite(L1, LOW);
  digitalWrite(L2, LOW);
  digitalWrite(R1, LOW);
  digitalWrite(R2, LOW);
  analogWrite(Lenable, 0);
  analogWrite(Renable, 0);
}
// spi boot stolen from
// Written by Nick Gammon
// February 2011
// taken from https://gammon.com.au/spi



char buf [64];
volatile uint8_t comm ; 
volatile byte pos;
volatile boolean process_it;
volatile boolean _boop;

_comms_packet_t the_packet;

void setup (void)
{
  Serial.begin (115200);   // debugging
  // turn on SPI in slave mode
  SPCR |= bit (SPE);
  // have to send on master in, *slave out*
  pinMode(MISO, OUTPUT);
  pinMode(SCK, INPUT_PULLUP);
  pinMode(MOSI, INPUT);
  pinMode(SS,INPUT_PULLUP);
  // get ready for an interrupt 
  pos = 0;   // buffer empty
  process_it = false;
  // now turn on interrupts
  SPI.attachInterrupt();
  buf [pos] = 0;  
  Serial.println("minibrain 0.1");
  enable();
  moveBot(false,50,15);
  disable();
}  // end of setup


// SPI interrupt routine
ISR (SPI_STC_vect)
{
  uint8_t c = SPDR;  // grab byte from SPI Data Register
  comms_input_byte(c);
  _boop = true;
  comm = SPDR;
}  // end of interrupt routine SPI_STC_vect

// main loop - wait for flag set in interrupt routine
void loop (void)
{
  if(_boop){
    Serial.println(comm);
    _boop = false;
  }

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
  }

  //if (process_it)
  if (false)
    {
        buf[pos] = 0;
        Serial.println("");
        Serial.print(buf);
        comm = buf[0];
        //Serial.print(int(comm),HEX);
        // Serial.print(int(comm),BIN);
        pos = 0;
        process_it = false;
        enable();
        switch (comm){
          case ']':
            sp = sp + 10;
            break;
          case '[':
            sp = sp - 10;
            break;
        //  case '}':
        //    len = len + 50;
        //    break;
        //////  case '{':
        //    len = len - 50;
        //    break;
          case 'w':
            moveBot(true,sp,len);
            stopMotors();
            break;
          case 's':
            moveBot(false,sp,len);
            stopMotors();
            break;
          case 'a':
            rotateBot(true,sp,len);
            stopMotors();
            break;
          case 'd':
            rotateBot(false,sp,len);
            stopMotors();
            break;
        }
        disable();
    }  
}  // end of loop
