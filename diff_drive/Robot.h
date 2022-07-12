#pragma once

/*
	Created:	2016-12-21
	Author:	Nahuel Taibo  savagemakers.com

	Extended:	2022-07-10
	Simon Kirkby 
*/

#include "Arduino.h"
#include "Configuration.h"
#include "L298NMotorDriver.h"
#include "DifferentialDriveController.h"
#include "HMC6352Compass.h"
#include "ACS712Current.h"

namespace SteveBot
{
	class Robot
	{
	public:
		Robot();
		void Update();
		void SetSpeed(int speed, int direction);
		void SetDiff(int left,int right);
		void SetJoy(int x,int y);
		void SetAcceleration(int acceleration);
		void SetTimeout(int timeout);
		void SetMinSpeed(int minspeed);
		void SetTrigger(int trigger);

	private:
		//We will use this variables to change the robot speed on after some seconds (without using delays)
		unsigned long previousTime = millis();
		unsigned int updateFreq = 5000;

		L298NMotorDriver rightMotor;
		L298NMotorDriver leftMotor;

		DifferentialDriveController difDrive;

		HMC6352Compass compass;
		ACS712Current currentSensor;

	};
};
