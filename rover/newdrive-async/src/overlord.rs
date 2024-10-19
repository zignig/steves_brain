/// This is the main scheduler and reflex generator
/// becuase these are async tasks they need to be decoupled
/// This means that there needs to be some channels an queues talk to stuff
/// This thing need to watch all the things and do stuff.
use core::task::Poll;
// use fugit::ExtU64;
use futures::{future::poll_fn, select_biased, FutureExt};

use crate::{channel, commands::Command, effectors::DriveCommands};

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
pub struct OverLord<'a> {
    state: SystemState,
    drive: channel::Sender<'a, DriveCommands>,
    to_spi: channel::Sender<'a,Command>,
    mode: Mode,
}

impl<'a> OverLord<'a> {
    pub fn new(drive: channel::Sender<'a, DriveCommands>,to_spi: channel::Sender<'a,Command>) -> Self {
        Self {
            state: SystemState::Init,
            drive: drive,
            to_spi: to_spi,
            mode: Mode::Wating,
        }
    }

    fn event(&mut self, com: Command) {
        match com {
            Command::Hello => {}
            Command::Stop => self.drive.send(DriveCommands::Stop),
            Command::Cont => {}
            Command::Run(l, r) => self.drive.send(DriveCommands::Run(l, r)),
            Command::SetAcc(rate) => self.drive.send(DriveCommands::SetRate(rate as i16)),
            Command::SetJoy(x, y) => self.drive.send(DriveCommands::Joy(x, y)),
            Command::SetTimeout(timeout) => self.drive.send(DriveCommands::SetTimeout(timeout)),
            Command::SetTrigger(_) => {}
            Command::SetMinspeed(_) => {}
            Command::SetMaxCurrent(_) => {}
            Command::Config => {}
            Command::Count => {}
            Command::Data(a, b, c, d) => {self.to_spi.send(Command::Data(a,b,c,d))}
            Command::Compass(_) => {}
            Command::GetMillis(_) => {}
            Command::Current(_) => {self.to_spi.send(Command::Hello)}
            Command::Verbose => {}
            Command::Fail => {}
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

    pub async fn task(
        &mut self,
        mut drive_outgoing: channel::Sender<'_, DriveCommands>,
        mut spi_outgoing: channel::Receiver<'_, Command>,
    ) {
        //
        loop {
            select_biased! {
                comm = spi_outgoing.receive().fuse() => {
                    crate::print!("{:?}",&comm);
                    self.event(comm);
                }

                _ = self.run_if().fuse() => {}
                complete => break
            }
            crate::print!("Run overlord event");
        }
    }
}
