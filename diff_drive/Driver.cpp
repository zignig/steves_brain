#include "Driver.h"

using namespace Taibot;

Driver::Driver(bool isEnabled, bool isVerbose)
{
	_isEnabled = isEnabled;
	_isVerbose = isVerbose;
}

bool Driver::IsEnabled()
{
	return _isEnabled;
}

bool Driver::IsVerbose()
{
	return _isVerbose;
}

void Driver::SetEnabled(bool isEnabled)
{
	_isEnabled = isEnabled;
}

void Driver::SetVerbose(bool isVerbose)
{
	_isVerbose = isVerbose;
}