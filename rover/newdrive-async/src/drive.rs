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
    Idle,
    Error
}

pub struct Drive{ 
    state: DriveState,
    counter: u32,
    reset: u32
}

impl Drive{ 
    pub fn new(counter: u32) -> Self{
        Self{
            state: DriveState::Idle,
            counter: counter,
            reset: counter
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
        crate::print!("counter {}",self.counter);
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
            } 
            time::delay(10.millis()).await;
        }
    }
}