#pragma once

/*
	Created:	2016-12-21
	Author:	Nahuel Taibo  savagemakers.com
*/

#include "Arduino.h"
#include "Configuration.h"
#include "L298NMotorDriver.h"
#include "DifferentialDriveController.h"
//#include "UltrasonicSensorArrayDriver.h"

namespace Taibot
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
	private:
		//We will use this variables to change the robot speed on after some seconds (without using delays)
		unsigned long previousTime = millis();
		unsigned int updateFreq = 5000;

		L298NMotorDriver rightMotor;
		L298NMotorDriver leftMotor;
		DifferentialDriveController difDrive;
		//UltrasonicSensorArrayDriver sonar; //The rest of the sonar configuration is placed in Configuration.h
	};
};
