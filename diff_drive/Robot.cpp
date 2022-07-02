#include "Robot.h"

using namespace SteveBot;

Robot::Robot() :
	rightMotor(true, false, PIN_L298N_ENA, PIN_L298N_IN1, PIN_L298N_IN2),
	leftMotor(true, false, PIN_L298N_ENB, PIN_L298N_IN3, PIN_L298N_IN4),
	difDrive(true, false, &rightMotor, &leftMotor),
        compass(true,true,0)
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
void SteveBot::Robot::SetDiff(int left, int right)
{
        difDrive.SetDiff(left,right);
}

void SteveBot::Robot::SetJoy(int x, int y)
{
        Serial.println("JOY ROBOT");
        difDrive.SetJoy(x,y);
}

void SteveBot::Robot::Update()
{
	difDrive.Update();
	//sonar.Update();
        compass.Update();
}
