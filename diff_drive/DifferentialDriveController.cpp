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
		_leftSpeed = 0;
		_rightSpeed= 0;
                UpdateMotorsSpeed();
                _lastUpdate = millis();
	}
        _leftMotor->Update();
        _rightMotor->Update();
}

void DifferentialDriveController::UpdateMotorsSpeed()
{
	if (IsVerbose())
	{
		Serial.print(F("DiffDrive: x="));
		Serial.print(F(" R="));
		Serial.print(_leftSpeed);
		Serial.print(F(" L="));
		Serial.println(_rightSpeed);
	}


        _rightMotor->SetSpeed(_leftSpeed);
        _leftMotor->SetSpeed(_rightSpeed);
}


int DifferentialDriveController::GetAcceleration()
{
	return _acceleration;
}

void DifferentialDriveController::SetSpeed(int x, int y)
{
    _lastUpdate = millis();
    _leftSpeed = x;
    _rightSpeed = y;
    UpdateMotorsSpeed();
}

int DifferentialDriveController::GetSpeed()
{
	return 0;
}

int DifferentialDriveController::GetDirection()
{
	return 0;
}

void DifferentialDriveController::SetDiff(int x, int y)
{
    SetSpeed(x,y);
    Serial.println("set diff");
}

void DifferentialDriveController::SetAcceleration(int acceleration)
{
        _rightMotor->SetAcceleration(acceleration);
        _leftMotor->SetAcceleration(acceleration);
}

void DifferentialDriveController::SetJoyStick(int x, int y)
{

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
        SetSpeed(rawLeft,rawRight);
}
