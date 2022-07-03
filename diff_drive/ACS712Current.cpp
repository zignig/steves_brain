#include "ACS712Current.h"
#include <Wire.h>
using namespace SteveBot;

ACS712Current::ACS712Current(bool isEnabled, bool isVerbose, int APin)
	: CurrentDriver(isEnabled, isVerbose),
	average(0) // Start the average readings for the bearing 
{
        // create the sensor 
        // set the pin
        _aPin = APin;
        
}

void ACS712Current::Update()
{
    GetCurrent();
}

int ACS712Current::GetCurrent()
{
	if (IsEnabled())
	{
		if (_lastReadingTime + MAX_READING_FREQUENCY_CURRENT <= millis())
		{
                        _latestCurrent = analogRead(_aPin);
                        float rescale =  -_latestCurrent + 512;
                        _latestCurrent = rescale *3.5 ; 
			average.Add(_latestCurrent);
			if (IsVerbose())
			{
				Serial.print(F("ACS712: current="));
				Serial.print(_latestCurrent);
				//Serial.print(average.Get());
				Serial.println(F(" units"));
			}
                        _lastReadingTime = millis();
		}

		return _latestCurrent;
	}
}

int ACS712Current::GetAverageCurrent()
{
	// Make the ping only if we shold
	GetCurrent();

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
