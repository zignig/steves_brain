//! Collect all the bits together for the main robot construct

// import some traits to make this easier;
use crate::shared::TankDrive;

// Create the grand robot construct
// perhaps the state machine goes in here ?
pub struct Robot<DR, CO, CM> {
    drive: DR,
    compass: CO,
    current: CM,
}

impl<DR: TankDrive, CO, CM> Robot<DR, CO, CM> {
    pub fn new(drive: DR, compass: CO, current: CM) -> Self {
        Self {
            drive,
            compass,
            current,
        }
    }

    pub fn drive(&mut self) {
        self.drive.enable();
    }

    pub fn runner(&mut self) {
        loop {}
    }
}
