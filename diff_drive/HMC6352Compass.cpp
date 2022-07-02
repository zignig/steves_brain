#include "HMC6352Compass.h"

using namespace SteveBot;

HMC6352Compass::HMC6352Compass(bool isEnabled, bool isVerbose, unsigned int maxDistance, unsigned int pinTrigger, unsigned int pinEcho)
	: CompassDriver(isEnabled, isVerbose, offet),
	average(maxDistance) // Start the average readings at the maxDistance value
{
	// Set the latest distance to the maximum, so the sensor does not start with a fake super close reading
	_latestDistance = maxDistance;
}

unsigned int HMC6352Compass::GetBearing()
{
	if (IsEnabled())
	{
		// We can only ping the sensor every 30ms, so we will do that, 
		// the rest of the calls to GetDistance will return the latest reading without pingig the sensor again  
		if (_lastReadingTime + MAX_READING_FREQUENCY <= millis())
		{
			// As enough time have passed, we can update the reading
			_latestDistance = sensor.ping_cm();

			// The NewPing library returns 0 when it does not detect an object.
			// We want this service to return the MaxDistance, so we dont get confuse this with a super close object
			if (_latestDistance == 0)
			{
				_latestDistance = GetMaxDistance();
			}

			average.Add(_latestDistance);

			if (IsVerbose())
			{
				// We will only print the distance when we got a real update
				Serial.print(F("HCSR04: d="));
				Serial.print(_latestDistance);
				Serial.println(F(" cm"));
			}
		}

		return _latestDistance;
	}
}

unsigned int HMC6352Compass::GetAverageBearing()
{
	// Make the ping only if we shold
	GetBearing();

	// Now get the latest average
	unsigned int latestAverage = average.Get();

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
