#pragma once  

namespace SteveBot
{
	class Driver
	{
	public:
		// isEnabled: Enables/Disables the Service  
		// isVerbose: Activates the logging output to Serial  
		Driver(bool isEnabled, bool isVerbose);

		// Use this method to know if the driver is enabled or not
		bool IsEnabled();

		// Use this method to know if the logging is is enabled or not before writing logs
		bool IsVerbose();

		// Enables/Disables the driver
		void SetEnabled(bool isEnabled);

		// Enables/Disables the logging
		void SetVerbose(bool isVerbose);
                
                // Run the Update 
                void Update();
	private:
		// Enable/Disable the service  
		bool _isEnabled = false;

		// Enable/Disable serial output for debugging 
		bool _isVerbose = false;
	};
};
