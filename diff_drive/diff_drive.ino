// STEVE's minibrain interface
// 
// SPI comms from the ESP32
// communications protocol 

#include "comms.h"
#include "settings.h"
#include "drive.h"

// Left Motor (A)
int enA = 3;
int in1 = 9;
int in2 = 8;
// Right Motor (B)
int enB = 5;
int in3 = 7;
int in4 = 6;
// speed and time length
int sp = 150;
int len = 500;

int inByte = 0 ;
int counter  = 0;



void enable()
{
  pinMode(enA, OUTPUT);
  pinMode(enB, OUTPUT);
  pinMode(in1, OUTPUT);
  pinMode(in2, OUTPUT);
  pinMode(in3, OUTPUT);
  pinMode(in4, OUTPUT);
 
}

void disable()
{
  pinMode(enA, INPUT);
  pinMode(enB, INPUT);
  pinMode(in1, INPUT);
  pinMode(in2, INPUT);
  pinMode(in3, INPUT);
  pinMode(in4, INPUT);
 
}

void moveBot(bool dir, int spd, int dur) {
  // Motor A
  digitalWrite(in1, !dir);
  digitalWrite(in2, dir);  //The '!' symbol inverts the boolean value. So for example, if dir is true, !dir is false.
  // Motor B
  digitalWrite(in3, !dir);
  digitalWrite(in4, dir);
  // Set motor speed to spd
  analogWrite(enA, spd);
  analogWrite(enB, spd);
  //Motion Duration
  delay(dur);
}

void rotateBot(bool dir, int spd, int dur) {
  // Motor A
  digitalWrite(in1, dir);
  digitalWrite(in2, !dir);  //The '!' symbol inverts the boolean value. So for example, if dir is true, !dir is false.
  // Motor B
  digitalWrite(in3, !dir);
  digitalWrite(in4, dir);
  // Set motor speed to spd
  analogWrite(enA, spd);
  analogWrite(enB, spd);
  //Rotation Duration
  delay(dur);
}

//Turn off both motors
void stopMotors() {
  digitalWrite(in1, LOW);
  digitalWrite(in2, LOW);
  digitalWrite(in3, LOW);
  digitalWrite(in4, LOW);
}
// spi boot stolen from
// Written by Nick Gammon
// February 2011
// taken from https://gammon.com.au/spi


#include <SPI.h>

char buf [64];
volatile char comm ; 
volatile byte pos;
volatile boolean process_it;

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
  moveBot(true,15,50);
  disable();
}  // end of setup


// SPI interrupt routine
ISR (SPI_STC_vect)
{
  byte c = SPDR;  // grab byte from SPI Data Register
  // add to buffer if room
  if (pos < (sizeof (buf) - 1))
    buf [pos++] = c;
  // example: newline means time to process buffer
  if ( c == '\n')
    process_it = true;
  //comm = SPDR;
  //process_it = true;
}  // end of interrupt routine SPI_STC_vect

// main loop - wait for flag set in interrupt routine
void loop (void)
{
  if (process_it)
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
