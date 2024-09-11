/// Mockup for the drive system

use core::{
    future::poll_fn,
    task::Poll,
};

use fugit::ExtU32;
use crate::channel::Receiver;
use futures::{select_biased, FutureExt};

use crate::time;

pub enum DriveState{
    Init,
    Running,
    Throttle(i16),
    Idle,
    Error
}

pub struct Drive{ 
    state: DriveState,
    counter: u32,
    reset: u32,
    throttle: i16
}

impl Drive{ 
    pub fn new(counter: u32) -> Self{
        Self{
            state: DriveState::Init,
            counter: counter,
            reset: counter,
            throttle: 0
        }
    }

    pub async fn run_if(&mut self){
        poll_fn(|_cx|{
            match self.state{
                DriveState::Init => {
                    crate::print!("Initialize the drive");
                    self.state = DriveState::Idle;
                    Poll::Pending
                },
                DriveState::Running => {
                    self.update();
                    Poll::Ready(())
                },
                DriveState::Throttle(val) =>{
                    self.throttle = val;
                    self.state = DriveState::Running;
                    Poll::Ready(())
                }
                DriveState::Idle => {
                    Poll::Pending
                },
                DriveState::Error => {
                    Poll::Pending
                },
            }
        } ).await
    }

    fn update(&mut self){
        self.counter -= 1; 
        crate::print!("drive {}",self.counter);
        if self.counter == 0{ 
            self.counter = self.reset;
            self.state = DriveState::Idle;
        }
    }

    pub async fn task(&mut self,mut incoming: Receiver<'_,DriveState>){
        loop {
            select_biased! {
                state = incoming.receive().fuse()=>{
                    self.state = state;
                }
                _ = self.run_if().fuse() => {}
                complete => break
            } 
            time::delay(5.millis()).await;
        }
    }
}