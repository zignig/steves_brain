#pragma once
#include "Driver.h"

namespace SteveBot
{

	class CompassDriver : public Driver
	{
	public:
		CompassDriver(bool isEnabled, bool isVerbose, int offset) :
			Driver(isEnabled, isVerbose)
		{
			_offset = offset;
		}


		virtual int GetBearing() = 0;

		virtual int GetAverageBearing() = 0;

	private:
		int _offset;
	};
};
