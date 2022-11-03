

pub trait Update {
    fn update(&mut self);
}

pub struct PeriodicUpdate<'a>{
    interval: u32,
    next_run: u32,
    action: &'a dyn Update,
}

use crate::serial_println;

impl<'a> PeriodicUpdate<'a>{ 
    pub fn new(interval:u32, action:& dyn Update,now:u32) -> Self {
        Self { 
            interval,
            next_run: now,
            action
        }
    }

    pub fn run(&self, now: u32){
        if now >= self.next_run {
            serial_println!("runner {}",self.next_run);
            self.action.update();
            self.next_run = now + self.interval;
        }
    }
}