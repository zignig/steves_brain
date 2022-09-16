	#include "DifferentialDriveController.h"

	using namespace SteveBot;

	DifferentialDriveController::DifferentialDriveController(bool isEnabled, bool isVerbose, MotorDriver* rightMotor, MotorDriver* leftMotor) 
		: Controller(isEnabled, isVerbose)
	{
		_rightMotor = rightMotor;
		_leftMotor = leftMotor;
	}

	void DifferentialDriveController::Update()
	{
		if ( _lastUpdate + _timeOut < millis()){
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

	void DifferentialDriveController::SetTimeout(int timeout)
	{
		_timeOut = timeout;
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
	}

	void DifferentialDriveController::SetAcceleration(int acceleration)
	{
		_rightMotor->SetAcceleration(acceleration);
		_leftMotor->SetAcceleration(acceleration);
	}

	void DifferentialDriveController::SetJoy(int x, int y)
	{

		float rawLeft;
		float rawRight;
		float magnitude;
		float rad;
		float fx = (float)x;
		float fy = (float)y;

		// maybe the joy should be rescaled 
		// not the raw drive.
		  
		magnitude = sqrt(fx * fx + fy * fy);
		rad = acos(abs(fx)/magnitude);
		
		if(isnan(rad)){
			rad= 0;
		} 

		float angle = rad * 180 / PI;
		float tcoeff = -1 + (angle / 90) * 2;
		float turn = tcoeff * abs(abs(fy) - abs(fx));
		turn = round(turn * 100) / 100;

		// And max of y or x is the movement
		float mov = max(abs(fy), abs(fx));

		// First and third quadrant
		if ((fx >= 0 && fy >= 0) || (fx < 0 && fy < 0))
		{
			rawLeft = mov; rawRight = turn;
		}
		else
		{
			rawRight = mov; rawLeft = turn;
		}

		// Reverse polarity
		if (fy < 0) {
			rawLeft = 0 - rawLeft;
			rawRight = 0 - rawRight;
		}

		// rescale to reduce twitch
		
		if ( rawLeft > _trigger)
		{
			rawLeft = map(rawLeft,0,255,_minSpeed,255);
		}
		if ( rawLeft < -_trigger)
		{
			rawLeft = map(rawLeft,0,-255,-_minSpeed,-255);
		}
		if ( rawRight > _trigger)
		{
			rawRight = map(rawRight,0,255,_minSpeed,255);

		}
		if ( rawRight < -_trigger)
		{
			rawRight = map(rawRight,0,-255,-_minSpeed,-255);
		}
		//Serial.println(rawLeft);
		//Serial.println(rawRight);
		SetSpeed(rawLeft,rawRight);
	}
