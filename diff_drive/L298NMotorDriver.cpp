#include "L298NMotorDriver.h"

using namespace Taibot;

L298NMotorDriver::L298NMotorDriver(bool isEnabled, bool isVerbose, unsigned int pinEnable, unsigned int pinInput1, unsigned int pinInput2) 
	: MotorDriver(isEnabled, isVerbose)
{
	_pinEnable = pinEnable;
	_pinInput1 = pinInput1;
	_pinInput2 = pinInput2;

	_currentSpeed = 0;
        pinMode(_pinEnable, OUTPUT);
        pinMode(_pinInput1, OUTPUT);
        pinMode(_pinInput2, OUTPUT);

}

void L298NMotorDriver::Update()
{
    unsigned long now = millis();
    if(( _lastUpdate + _stepTick )< now )
    {
        if(_currentSpeed < _targetSpeed)
        {
            _currentSpeed += _acceleration; 
            // too far
            if(_currentSpeed > _targetSpeed){
                _currentSpeed = _targetSpeed;
            }
            SetTarget(_currentSpeed);
        }
        if(_currentSpeed > _targetSpeed)
        {
            _currentSpeed -= _acceleration; 
            // too far
            if(_currentSpeed < _targetSpeed){
                _currentSpeed = _targetSpeed;
            }
            SetTarget(_currentSpeed);
        }
        _lastUpdate = now;
    }     
}

void L298NMotorDriver::SetSpeed(int speed)
{
    _lastUpdate = millis();
    _targetSpeed = speed;
}

void L298NMotorDriver::SetAcceleration(int acceleration)
{
    _acceleration = acceleration;
}

void L298NMotorDriver::SetTarget(int speed)
{
	if (IsVerbose())
	{
		//If we are logging, print the speed we are giving to the motor
		Serial.print(F("L298NMotor: speed:"));
		Serial.println(speed);
	}

	// Only activate the motors if the driver is enabled
	if (IsEnabled())
	{
		if (speed >= 0)
		{
			// if the speed is positive or 0 then move forward
			analogWrite(_pinEnable, speed);
			digitalWrite(_pinInput1, HIGH);
			digitalWrite(_pinInput2, LOW);
		}
		else
		{
			// if the speed is negative then move backwards
			analogWrite(_pinEnable, -speed);
			digitalWrite(_pinInput1, LOW);
			digitalWrite(_pinInput2, HIGH);
		}
	}
}


int L298NMotorDriver::GetSpeed() const
{
	return _currentSpeed;
}
