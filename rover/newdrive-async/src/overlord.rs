/// This is the main scheduler and reflex generator
/// becuase these are async tasks they need to be decoupled
/// This means that there needs to be some channels an queues talk to stuff
/// This thing need to watch all the things and do stuff.
use core::task::Poll;
// use fugit::ExtU64;
use futures::{future::poll_fn, select_biased, FutureExt};

use crate::{channel, commands::Command};

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
    Wating,
}
pub struct OverLord {
    state: SystemState,
    mode: Mode,
}

impl OverLord {
    pub fn new() -> Self {
        Self {
            state: SystemState::Init,
            mode: Mode::Wating,
        }
    }

    fn event(&mut self, com: Command){
        match com{
            Command::Hello => todo!(),
            Command::Stop => todo!(),
            Command::Cont => todo!(),
            Command::Run(_, _) => todo!(),
            Command::SetAcc(_) => todo!(),
            Command::SetJoy(_, _) => todo!(),
            Command::SetTimeout(_) => todo!(),
            Command::SetTrigger(_) => todo!(),
            Command::SetMinspeed(_) => todo!(),
            Command::SetMaxCurrent(_) => todo!(),
            Command::Config => todo!(),
            Command::Count => todo!(),
            Command::Data(_, _, _, _) => todo!(),
            Command::Compass(_) => todo!(),
            Command::GetMillis(_) => todo!(),
            Command::Current(_) => todo!(),
            Command::Verbose => todo!(),
            Command::Fail => todo!(),
        }
    }

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

    pub async fn task(&mut self, mut spi_outgoing: channel::Receiver<'_, Command>) {
        //
        loop {
            select_biased! {
                comm = spi_outgoing.receive().fuse() => {
                    crate::print!("{:?}",&comm);
                    // self.event(comm);
                }
                _ = self.run_if().fuse() => {}
                complete => break
            }
            crate::print!("Run overlord event");
        }
    }
}
