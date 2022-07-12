#pragma once
#include "Controller.h"
#include "MotorDriver.h"

#define SPEED_UPDATE_FREQ 100

namespace SteveBot
{
	struct DifferentialVector
	{
		int Right;
		int Left;
	};

	class DifferentialDriveController : public Controller
	{
	public:
		DifferentialDriveController(bool isEnabled, bool isVerbose, MotorDriver* rightMotor, MotorDriver* leftMotor);

		void Update();

		void SetAcceleration(int acceleration);
		int GetAcceleration();
		void SetTimeout(int timeout);
		void SetMinSpeed(int minspeed);
		void SetTrigger(int trigger);

		// Sets the required speed and direction of movement
		// speed: valid range between 0 and 255
		// direction: between 0 and 360 degrees
		void SetSpeed(int speed, int direction);
		// calculate joystick to diff drive calcs
		void SetJoy(int x, int y);
		// drive the motors individually
		void SetDiff(int x, int y);

		int GetSpeed();
		int GetDirection();

	private:
		MotorDriver* _rightMotor;
		MotorDriver* _leftMotor;

		// Acceleration (in speed units per 100 ms) used to modify the _currentSpeed, until we get to the _targetSpeed
		// Range goes between 1 and 255
		// If 255, the speed will be instantly assigned to the motors
		// ie: If _acceleration=1, _currentspeed will be increased in 1, every 100 ms, so if we start at _currentspeed=0 and _targetSpeed=255, it will take 25500ms (255*100) to get to full speed
		// ie: If _acceleration=10, _currentspeed will be increased in 10, every 100 ms, so if we start at _currentspeed=0 and _targetSpeed=255, it will take 2550ms ((255/10)*100) to get to full speed
		// ie: If _acceleration=100, _currentspeed will be increased in 100, every 100 ms, so if we start at _currentspeed=0 and _targetSpeed=255, it will take 255ms ((255/100)*100) to get to full speed
		unsigned int _acceleration = 20;
		int _trigger = 10;
		int _minSpeed = 70;
		int _timeOut = 500;

		// The last speed set to the motors
        int _leftSpeed;
		int _rightSpeed;
		// The speed we want to achieve, (we will get to this _targetSpeed ant _acceleration increments per second) 

		unsigned long _lastSpeedUpdate = 0;

		//calculates the speed each motor should have according with the calculated _currentspeed and updates them
		void UpdateMotorsSpeed();

		
		// timeout for drive commands
		unsigned long _lastUpdate;
	};
};
