#include "HMC6352Compass.h"
#include <Wire.h>
using namespace SteveBot;

HMC6352Compass::HMC6352Compass(bool isEnabled, bool isVerbose, int offset)
	: CompassDriver(isEnabled, isVerbose, offset),
	average(0) // Start the average readings for the bearing 
{
        // create the compass
        SlaveAddress = SlaveAddress >> 1;
        Wire.begin();
}

void HMC6352Compass::Update()
{
    GetBearing();
}

int HMC6352Compass::GetBearing()
{
	if (IsEnabled())
	{
		// We can only ping the sensor every 30ms, so we will do that, 
		// the rest of the calls to GetDistance will return the latest reading without pingig the sensor again  
		if (_lastReadingTime + MAX_READING_FREQUENCY <= millis())
		{
                        
			// As enough time have passed, we can update the reading
			//_latestBearing= sensor.ping_cm();
                        Wire.beginTransmission(SlaveAddress);
                        Wire.write(ReadAddress);
                        Wire.endTransmission();
                        delay(6);
                        Wire.requestFrom(SlaveAddress,2);
                        
                        byte MSB = Wire.read();
                        byte LSB = Wire.read();
                        float headingSum = (MSB << 8) + LSB;
                        _latestBearing = headingSum / 10;

			average.Add(_latestBearing);

			if (IsVerbose())
			{
				// We will only print the distance when we got a real update
				Serial.print(F("HMC6426: bearing="));
				Serial.print(_latestBearing);
				Serial.println(F(" deg"));
			}
                        _lastReadingTime = millis();
		}

		return _latestBearing;
	}
}

int HMC6352Compass::GetAverageBearing()
{
	// Make the ping only if we shold
	GetBearing();

	// Now get the latest average
	int latestAverage = average.Get();

	if (IsVerbose())
	{
		// We will only print the distance when we got a real update
		Serial.print(F("HMC6352: a="));
		Serial.print(latestAverage);
		Serial.println(F(" deg"));
	}

	// return the average of the latest pings
	return latestAverage;
}
