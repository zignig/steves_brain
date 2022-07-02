#pragma once

#include <Arduino.h>
#include "CompassDriver.h"
#include "MovingAverage.h"

#define MAX_READING_FREQUENCY 30 //The maximum frequency the HC-SR04 can be queried at

namespace SteveBot
{

	class HMC6352Compass : public CompassDriver 
	{

	public:
		HMC6352Compass(bool isEnabled, bool isVerbose, unsigned int maxDistance, unsigned int pinTrigger, unsigned int pinEcho);

		unsigned int GetBearing();

		unsigned int GetAverageBearing();

	private:
		// The last time we made a reading
		unsigned long _lastReadingTime = 0;

		// The last reading we made
		unsigned int _latestBearing;

		// We will be averaging the last 3 reading of the sensor, to get a more reallistic reading
		MovingAverage<unsigned int, 3> average;
	};
};
