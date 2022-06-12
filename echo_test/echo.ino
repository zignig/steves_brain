// Written by Nick Gammon
// February 2011
// taken from https://gammon.com.au/spi


#include <SPI.h>

char buf [128];
volatile byte pos;
volatile boolean process_it;

void setup (void)
{
  Serial.begin (115200);   // debugging
  // turn on SPI in slave mode
  SPCR |= bit (SPE);
  // have to send on master in, *slave out*
  pinMode(MISO, OUTPUT);
  // pinMode(SS,INPUT_PULLUP);
  // get ready for an interrupt 
  pos = 0;   // buffer empty
  process_it = false;
  // now turn on interrupts
  SPI.attachInterrupt();
  Serial.println("hello from echo");
}  // end of setup


// SPI interrupt routine
ISR (SPI_STC_vect)
{
  byte c = SPDR;  // grab byte from SPI Data Register
  // add to buffer if room
  if (pos < (sizeof (buf) - 1))
    buf [pos++] = c;
  // example: newline means time to process buffer
  if (  c == '\n')
    process_it = true;
}  // end of interrupt routine SPI_STC_vect

// main loop - wait for flag set in interrupt routine
void loop (void)
{
  if (process_it)
    {
        buf[pos] = 0;  
        Serial.println (buf);
        pos = 0;
        process_it = false;
    }  // end of flag set
}  // end of loop
