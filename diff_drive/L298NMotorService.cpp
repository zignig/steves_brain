#include "L298NMotorService.h"


using namespace Stevebot;

L298NMotorService::L298NMotorService(bool isEnabled, bool isVerbose, unsigned int pinEnable, unsigned int pinIn1, unsigned int pinIn2)
	: MotorService(isEnabled, isVerbose)
{
	_pinEnable = pinEnable;
	_pinIn1 = pinIn1;
	_pinIn2 = pinIn2;

	_currentSpeed = 0;
}

void L298NMotorService::SetTarget(int speed)
{
        _currentSpeed = speed;
	if (IsVerbose())
	{
		//If we are logging, print the speed we are giving to the motor
                if(_currentSpeed != 0){
                    Serial.print(F("L298NMotor: speed="));
                    Serial.println(speed);
                }
	}

	// Only activate the motors if the driver is enabled
	if (IsEnabled())
	{
		if (speed >= 0)
		{
			// if the speed is positive or 0 then move forward
			analogWrite(_pinEnable, speed);
			digitalWrite(_pinIn1, HIGH);
			digitalWrite(_pinIn2, LOW);
		}
		else
		{
			// if the speed is negative then move backwards
			analogWrite(_pinEnable, -speed);
			digitalWrite(_pinIn1, LOW);
			digitalWrite(_pinIn2, HIGH);
		}
	} else {
            analogWrite(_pinEnable,0);
        }
}

void L298NMotorService::SetSpeed(int speed)
{
        // command time out ( 1 second for now) 
        _lastUpdate = millis();
	// Save the current speed...
        _targetSpeed = speed;
}

int L298NMotorService::GetSpeed() const
{
	return _currentSpeed;
}

void L298NMotorService::Setup()
{
  // Set the Pin modes
  pinMode(_pinEnable, OUTPUT);
  pinMode(_pinIn1, OUTPUT);
  pinMode(_pinIn2, OUTPUT);
}

void L298NMotorService::Update()
{
    unsigned long now = millis();
    // Do a linear speed slide and timeout
    // Stop motor after 1 second of no speed set
    if(_lastUpdate + _timeout < now ){
        SetSpeed(0);
        _lastUpdate = now;
    }
    // Step ramping
    if(_lastRamp + _rampInterval < now){
        _lastRamp = now;
        int diff = _targetSpeed - _currentSpeed;
        if( diff < 0){
            if( abs(diff) < _step){
                _currentSpeed -= abs(diff) ;
            } else {
                _currentSpeed -= _step;    
            }
        }
        if( diff > 0){
            if( abs(diff) < _step){
                _currentSpeed +=  abs(diff) ;
            } else {
                _currentSpeed += _step;    
            }            
        }
        SetTarget(_currentSpeed);
    }
}
