#pragma once
#include "Driver.h"

namespace SteveBot
{

	class DistanceSensorDriver : public Driver
	{
	public:
		DistanceSensorDriver(bool isEnabled, bool isVerbose, unsigned int maxDistance) :
			Driver(isEnabled, isVerbose)
		{
			_maxDistance = maxDistance;
		}

		unsigned int GetMaxDistance()
		{
			return _maxDistance;
		}

		virtual unsigned int GetDistance() = 0;

		virtual unsigned int GetAverageDistance() = 0;

	private:
		unsigned int _maxDistance;
	};
};
