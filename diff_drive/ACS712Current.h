#pragma once

#include <Arduino.h>
#include <Wire.h>
#include "CurrentDriver.h"
#include "MovingAverage.h"

#define MAX_READING_FREQUENCY_CURRENT 250 

namespace SteveBot
{

	class ACS712Current : public CurrentDriver 
	{

	public:
		ACS712Current(bool isEnabled, bool isVerbose, int APin);
                
                void Update();
		int GetCurrent();
		int GetAverageCurrent();

	private:
                //ACS712 Current sensor ;
		// The last time we made a reading
		unsigned long _lastReadingTime = 0;

		// The last reading we made
		int _latestCurrent = 0;
                int _aPin =0;
		MovingAverage<int, 3> average;
	};
};
