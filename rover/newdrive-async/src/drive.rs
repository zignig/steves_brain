/// Mockup for the drive system
use core::{future::poll_fn, task::Poll};

use crate::{
    channel::{self, Receiver},
    time::TickDuration,
    time::Ticker,
};

use fugit::ExtU32;
use futures::{select_biased, FutureExt};

use crate::time;

// Overstate of the drive
#[derive(PartialEq)]
pub enum DriveState {
    Init,
    Running,
    Idle,
    Error,
}

// Driver commands
pub enum DriveCommands {
    Forward,
    Backwards,
    Left,
    Right,
    Stop,
}

pub struct Drive {
    state: DriveState,
    timeout: TickDuration,
    next_timeout: u32,
    throttle: i16,
    current: i16,
    rate: i16,
}

impl Drive {
    pub fn new(timeout: TickDuration) -> Self {
        Self {
            state: DriveState::Init,
            timeout: timeout,
            next_timeout: 0,
            throttle: 0,
            current: 0,
            rate: 5,
        }
    }

    pub async fn run_if(&mut self) {
        poll_fn(|_cx| match self.state {
            DriveState::Init => {
                crate::print!("Initialize the drive");
                self.state = DriveState::Idle;
                Poll::Pending
            }
            DriveState::Running => {
                self.update();
                Poll::Ready(())
            }
            DriveState::Idle => Poll::Pending,
            DriveState::Error => Poll::Pending,
        })
        .await
    }

    fn update(&mut self) {
        if Ticker::ticks() > self.next_timeout {
            crate::print!("drive timeout");
            self.current = 0;
            self.state = DriveState::Idle;
            return;
        }
        if self.current != self.throttle {
            if self.current < self.throttle {
                self.current += self.rate;
                if self.current > self.throttle {
                    self.current = self.throttle;
                }
            }
            if self.current > self.throttle {
                self.current -= self.rate;
                if self.current < self.throttle {
                    self.current = self.throttle
                }
            }
        }
        crate::print!("current {}", self.current);
    }

    fn enable(&mut self) {
        self.next_timeout = Ticker::ticks() + self.timeout.ticks();
        self.state = DriveState::Running;
    }

    fn disable(&mut self) {
        self.state = DriveState::Idle;
    }

    fn set_command(&mut self, command: DriveCommands) {
        // update as the update is lost in the select_biased
        self.update();
        // only run in running and idle
        if (self.state == DriveState::Running) | (self.state == DriveState::Idle) {
            match command {
                DriveCommands::Forward => {
                    self.enable();
                    self.throttle = 200;
                    crate::print!("forward")
                }
                DriveCommands::Backwards => {
                    self.enable();
                    self.throttle = -200;
                    crate::print!("backwards")
                }
                DriveCommands::Left => {
                    self.enable();
                    crate::print!("left")
                }
                DriveCommands::Right => {
                    self.enable();
                    crate::print!("right")
                }
                DriveCommands::Stop => {
                    self.disable();
                    self.throttle = 0;
                    crate::print!("stop");
                }
            }
        }
    }

    pub async fn task(
        &mut self,
        mut set_state: channel::Receiver<'_, DriveState>,
        mut commands: channel::Receiver<'_, DriveCommands>,
    ) {
        loop {
            select_biased! {
                state = set_state.receive().fuse()=>{
                    crate::print!("state change");
                    self.state = state;
                }
                command = commands.receive().fuse()=>{
                    self.set_command(command);
                }
                _ = self.run_if().fuse() => {}
                complete => break
            }
            time::delay(15.millis()).await;
        }
    }
}
