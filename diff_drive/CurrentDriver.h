#pragma once
#include "Driver.h"

namespace SteveBot
{

	class CurrentDriver: public Driver
	{
	public:
		CurrentDriver(bool isEnabled, bool isVerbose) :
			Driver(isEnabled, isVerbose)
		{
		}


		virtual int GetCurrent() = 0;

		virtual int GetAverageCurrent() = 0;

	private:
	};
};
