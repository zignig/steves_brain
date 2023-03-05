// The various parts of the joystick reader

use arduino_hal::adc::Channel;

// Single axis

pub struct Axis {
    zero: i16,
    value: i16,
    min: i16,
    max: i16,
    // ? scaling factors
    // ? linearization
    // ?
}

impl Axis {
    fn default() -> Self {
        Self {
            zero: 0,
            value: 0,
            min: -1000,
            max: 1000,
        }
    }
}

pub struct Joy3Axis {
    x: Axis,
    y: Axis,
    z: Axis,
}

impl Default for Joy3Axis {
    fn default() -> Self {
        Self {
            x: Axis::default(),
            y: Axis::default(),
            z: Axis::default(),
        }
    }
}
