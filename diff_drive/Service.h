// stolen from savage maker 
// https://bitbucket.org/savagemakers/taibot.git

#pragma once  

namespace Stevebot 
{
	class Service
	{
	public:
		// isEnabled: Enables/Disables the Service  
		// isVerbose: Activates the logging output to Serial  
		Service(bool isEnabled, bool isVerbose);

		// Use this method to know if the driver is enabled or not
		bool IsEnabled();

		// Use this method to know if the logging is is enabled or not before writing logs
		bool IsVerbose();

		// Enables/Disables the driver
		void SetEnabled(bool isEnabled);

		// Enables/Disables the logging
		void SetVerbose(bool isVerbose);

		// Needs to be implemented by every service, so we keep a convention
		virtual void Setup() = 0;

		// Needs to be implemented by every service, so we keep a convention
		virtual void Update() = 0;

	private:
		// Enable/Disable the service  
		bool _isEnabled = false;

		// Enable/Disable serial output for debugging 
		bool _isVerbose = false;
	};
};
