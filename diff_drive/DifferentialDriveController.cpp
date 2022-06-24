#include "DifferentialDriveController.h"

using namespace Taibot;

DifferentialDriveController::DifferentialDriveController(bool isEnabled, bool isVerbose, MotorDriver* rightMotor, MotorDriver* leftMotor) 
	: Controller(isEnabled, isVerbose)
{
	_rightMotor = rightMotor;
	_leftMotor = leftMotor;
}

void DifferentialDriveController::Update()
{
        if ( _lastUpdate + _timeout < millis()){
            _targetSpeed= 0;
        }
	if (_currentspeed != _targetSpeed) //Only update the _currentspeed if its different from _targetSpeed
	{
		// If its time to update the _currentspeed
		if ((_lastSpeedUpdate + SPEED_UPDATE_FREQ) < millis())
		{
			_lastSpeedUpdate = millis();

			if (_currentspeed < _targetSpeed) // We have to accelerate...
			{
				_currentspeed += _acceleration;

				// In case we accelerated too much
				if (_currentspeed > _targetSpeed)
				{
					_currentspeed = _targetSpeed;
				}
			}
			else if (_currentspeed > _targetSpeed) // We have to deaccelerate...
			{
				_currentspeed -= _acceleration;

				// In case we deaccelerated too much
				if (_currentspeed < _targetSpeed)
				{
					_currentspeed = _targetSpeed;
				}
			}

			UpdateMotorsSpeed();
		}
	}
}

void DifferentialDriveController::UpdateMotorsSpeed()
{
	DifferentialVector driveVector = CalculateDifferentialDrive(_currentspeed, _direction);

	if (IsVerbose())
	{
		Serial.print(F("DiffDrive: current="));
		Serial.print(_currentspeed);
		Serial.print(F(" dir="));
		Serial.print(_direction);
		Serial.print(F(" R="));
		Serial.print(driveVector.Right);
		Serial.print(F(" L="));
		Serial.println(driveVector.Left);
	}


	_rightMotor->SetSpeed(driveVector.Right);
        _leftMotor->SetSpeed(driveVector.Left);
}

void DifferentialDriveController::SetAcceleration(int acceleration)
{
	_acceleration = acceleration;
}

int DifferentialDriveController::GetAcceleration()
{
	return _acceleration;
}

void DifferentialDriveController::SetSpeed(int speed, int direction)
{
        _lastUpdate = millis();
	_targetSpeed = speed;
	_direction = direction;
}

int DifferentialDriveController::GetSpeed()
{
	return _currentspeed;
}

int DifferentialDriveController::GetDirection()
{
	return _direction;
}

DifferentialVector DifferentialDriveController::CalculateDifferentialDrive(int x, int y)
{
	DifferentialVector result;

	float rawLeft;
	float rawRight;
        float magnitude;
        float angle;

        magnitude = sqrt(x * x + y * y);
        angle = acos(abs(x)/magnitude);
        
        if(isnan(angle)){
            angle = 0;
        }        

	float tcoeff = -1 + (angle / 90) * 2;
	float turn = tcoeff * abs(abs(y) - abs(x));
	turn = round(turn * 100) / 100;

	// And max of y or x is the movement
	float mov = max(abs(y), abs(x));

	// First and third quadrant
	if ((x >= 0 && y >= 0) || (x < 0 && y < 0))
	{
		rawLeft = mov; rawRight = turn;
	}
	else
	{
		rawRight = mov; rawLeft = turn;
	}

	// Reverse polarity
	if (y < 0) {
		rawLeft = 0 - rawLeft;
		rawRight = 0 - rawRight;
	}

	result.Right = rawLeft;
	result.Left = rawRight;

	return result;
}
