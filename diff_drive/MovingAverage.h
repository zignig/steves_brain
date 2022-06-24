#pragma once

template <typename V, int N> class MovingAverage
{
public:
	/*
	* @brief Class constructor.
	* @param n the size of the moving average window.
	* @param def the default value to initialize the average.
	*/
	MovingAverage(V def = 0) : _sum(0), p(0)
	{
		for (int i = 0; i < N; i++)
		{
			_samples[i] = def;
			_sum += _samples[i];
		}
	}

	/*
	* @brief Add a new sample.
	* @param new_sample the new sample to add to the moving average.
	* @return the updated average.
	*/
	V Add(V newSample)
	{
		_sum = _sum - _samples[p] + newSample;
		_samples[p++] = newSample;
		if (p >= N)
		{
			p = 0;
		}

		_lastAverage = _sum / N;

		return _lastAverage;
	}

	V Get() const
	{
		return  _lastAverage;
	}

private:
	V _samples[N];
	V _sum;
	unsigned int p;
	V _lastAverage;
};