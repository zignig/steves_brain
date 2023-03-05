#pragma once

#include <Arduino.h>
#include <Wire.h>
#include "CompassDriver.h"
#include "MovingAverage.h"

#define MAX_READING_FREQUENCY 100 

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
                //hmc6352 Compass;
		// The last time we made a reading
		unsigned long _lastReadingTime = 0;
                int SlaveAddress = 0x42;
                int ReadAddress = 0x41;

		// The last reading we made
		int _latestBearing;

		MovingAverage<unsigned int, 3> average;
	};
};
