#pragma once

#include <Arduino.h>
#include "CompassDriver.h"
#include "MovingAverage.h"
#include "hmc6352.h"

#define MAX_READING_FREQUENCY 500 

namespace SteveBot
{

	class HMC6352Compass : public CompassDriver 
	{

	public:
		HMC6352Compass(bool isEnabled, bool isVerbose, int offset );
                
                void Update();
		int GetBearing();
		int GetAverageBearing();

	private:
		// The last time we made a reading
		unsigned long _lastReadingTime = 0;

		// The last reading we made
		int _latestBearing;

		// We will be averaging the last 3 reading of the sensor, to get a more reallistic reading
		MovingAverage<unsigned int, 3> average;
	};
};
