#include "HMC6352Compass.h"
#include "hmc6352.h"
using namespace SteveBot;

HMC6352Compass::HMC6352Compass(bool isEnabled, bool isVerbose, int offset)
	: CompassDriver(isEnabled, isVerbose, offset),
	average(0) // Start the average readings for the bearing 
{
        // create the compass
        
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
