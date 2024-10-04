/// This is the main scheduler and reflex generator
/// becuase these are async tasks they need to be decoupled
/// This means that there needs to be some channels an queues talk to stuff
/// This thing need to watch all the things and do stuff.

use core::task::Poll;
use fugit::ExtU64;
use futures::{future::poll_fn, select_biased, FutureExt};

use crate::time;

#[allow(dead_code)]
enum SystemState {
    Init,
    Running,
    Idle,
    Shutdown,
    Error,
    Fatal,
}

enum Mode { 
    Directed,
    Auto,
    Calibrate,
    Wating
}
pub struct OverLord {
    state: SystemState,
    mode: Mode
}

impl OverLord {
    pub fn new() -> Self {
        Self {
            state: SystemState::Init,
            mode: Mode::Wating
        }
    }

    // TODO
    pub async fn run_if(&mut self) {
        poll_fn(|_cx| match self.state {
            SystemState::Init => {
                // Init the devices as needed
                crate::print!("Initialize the droid");
                self.state = SystemState::Idle;
                Poll::Ready(())
            }
            SystemState::Running => Poll::Pending,
            SystemState::Idle => Poll::Pending,
            SystemState::Shutdown => Poll::Pending,
            SystemState::Error => Poll::Pending,
            SystemState::Fatal => Poll::Pending,
        })
        .await
    }

    pub async fn task(&mut self) {
        //  
        loop {
            select_biased! {
                _ = self.run_if().fuse() => {}
                _ = time::delay(10.secs()).fuse() => {} 
                complete => break
            }
            crate::print!("Appease the overlord");
        }
    }
}
