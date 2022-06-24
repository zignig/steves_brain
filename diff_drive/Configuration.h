#pragma once

#include "Arduino.h"

// Pin definitions for the L298N Motor driver 
// Change this defines according to your hardware configuration
#define PIN_L298N_ENA 3
#define PIN_L298N_IN1 9
#define PIN_L298N_IN2 8
#define PIN_L298N_IN3 7
#define PIN_L298N_IN4 6
#define PIN_L298N_ENB 5


// PIN CONFIGURATION - Sonar Array
#define PIN_HCSR04_0 34
#define PIN_HCSR04_1 45
#define PIN_HCSR04_2 35
#define PIN_HCSR04_3 44
#define PIN_HCSR04_4 36
#define PIN_HCSR04_5 43
#define SONAR_ARRAY_ANGLES { 75, 105, 45, 135, 15, 165 }

// Pin Definitions for the HC-SR04 DistanceSensor
#define MAX_RANGEFINDER_DISTANCE 200
