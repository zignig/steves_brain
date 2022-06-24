#pragma once
namespace Taibot
{
	class Controller
	{
	public:
		// isEnabled: Enables/Disables the Service  
		// isVerbose: Activates the logging output to Serial  
		Controller(bool isEnabled, bool isVerbose)
		{
			_isEnabled = isEnabled;
			_isVerbose = isVerbose;
		}

		// Use this method to know if the driver is enabled or not
		bool IsEnabled()
		{
			return _isEnabled;
		}

		// Use this method to know if the logging is is enabled or not before writing logs
		bool IsVerbose()
		{
			return _isVerbose;
		}

		// Enables/Disables the driver
		void SetEnabled(bool isEnabled)
		{
			_isEnabled = isEnabled;
		}

		// Enables/Disables the logging
		void SetVerbose(bool isVerbose)
		{
			_isVerbose = isVerbose;
		}

		// Should be called on the main loop function
		virtual void Update() = 0;

	private:
		// Enable/Disable the service  
		bool _isEnabled = false;

		// Enable/Disable serial output for debugging 
		bool _isVerbose = false;
	};
};
