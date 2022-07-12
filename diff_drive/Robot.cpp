#include "Robot.h"

using namespace SteveBot;

Robot::Robot() :
	rightMotor(true, false, PIN_L298N_ENA, PIN_L298N_IN1, PIN_L298N_IN2),
	leftMotor(true, false, PIN_L298N_ENB, PIN_L298N_IN3, PIN_L298N_IN4),
	difDrive(true, false, &rightMotor, &leftMotor),
        compass(true,false,0),
        currentSensor(true,true,PIN_CURRENT)
{
}


void SteveBot::Robot::SetSpeed(int speed, int direction)
{
        difDrive.SetSpeed(speed,direction);
}

void SteveBot::Robot::SetAcceleration(int acceleration)
{
        difDrive.SetAcceleration(acceleration);
}

void SteveBot::Robot::SetTimeout(int timeout)
{
        difDrive.SetTimeout(timeout);
}

void SteveBot::Robot::SetTrigger(int trigger)
{
        difDrive.SetTrigger(trigger);
}

void SteveBot::Robot::SetMinSpeed(int minspeed)
{
        difDrive.SetMinSpeed(minspeed);
}

void SteveBot::Robot::SetDiff(int left, int right)
{
        difDrive.SetDiff(left,right);
}

void SteveBot::Robot::SetJoy(int x, int y)
{
        //Serial.println("JOY ROBOT");
        difDrive.SetJoy(x,y);
}

void SteveBot::Robot::Update()
{
        // Primary update loop
	difDrive.Update();
        compass.Update();
        currentSensor.Update();
}
