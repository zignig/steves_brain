#include "Service.h"

using namespace Stevebot;

Service::Service(bool isEnabled, bool isVerbose)
{
	_isEnabled = isEnabled;
	_isVerbose = isVerbose;
}

bool Service::IsEnabled()
{
	return _isEnabled;
}

bool Service::IsVerbose()
{
	return _isVerbose;
}

void Service::SetEnabled(bool isEnabled)
{
	_isEnabled = isEnabled;
}

void Service::SetVerbose(bool isVerbose)
{
	_isVerbose = isVerbose;
}
