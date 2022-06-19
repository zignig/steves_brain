#pragma once
#include "Arduino.h"
#include "Service.h"

namespace Stevebot
{
	class MotorService : public Service
	{
	public:
		MotorService(bool isEnabled, bool isVerbose);

		// Sets the speed of the motor
		// speed = 0 then  motor stops
		// speed between -255 and -1 then motor runs backwards 
		// speed between 1 and 255 then motor runs forward
		// Method that must be implemented in every MotorService specific implementation inheriting from the MotorService class
		virtual void SetSpeed(int speed) = 0;

		// Returns the current speed of the motor
		// Method that must be implemented in every MotorService specific implementation inheriting from the MotorService class
		virtual int GetSpeed() const = 0;
	};
};
