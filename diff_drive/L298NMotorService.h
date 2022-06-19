#pragma once
#include "MotorService.h"

namespace Stevebot 
{
	class L298NMotorService : public MotorService
	{
	public:
		L298NMotorService(bool isEnabled, bool isVerbose, unsigned int pinEnable, unsigned int pinIn1, unsigned int pinIn2);

		// Implements the method inherited from the base MotorService class
		void SetSpeed(int speed);

		// Implements the method inherited from the base MotorService class
		int GetSpeed() const;

		void Setup();

		void Update();
			
	private:
                void SetTarget(int speed);
		unsigned int _pinEnable;
		unsigned int _pinIn1;
		unsigned int _pinIn2;

                int _step = 5;
                unsigned long _lastUpdate = 0;
                unsigned long _timeout = 1000;
                unsigned long _rampInterval = 15;
                unsigned long _lastRamp = 0;
                
                int _targetSpeed = 0;

		// Keeps track of the current speed of the Motor driver
		int _currentSpeed = 0;
	};
};
