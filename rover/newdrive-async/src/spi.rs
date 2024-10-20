//! Serialize Commands to and from the SPI interface.
//!

use arduino_hal::pac::SPI;
use avr_device::interrupt::Mutex;
use core::{cell::RefCell, future::poll_fn, task::Poll};
use futures::{select_biased, FutureExt};

use crate::{
    channel,
    commands::Command,
    isrqueue::{self, ISRQueue},
    print,
    queue::Queue,
};

pub const FRAME_SIZE: usize = 8;
/// Frame Header
pub const SYNC1: u8 = 0xF;
pub const SYNC2: u8 = 0xE;

// Frame Buffer
#[derive(Clone, Copy, PartialEq)]
pub enum FrameStatus {
    Running,
    Idle,
}

#[derive(Clone, Copy)]
pub struct FrameBuffer {
    pub data: [u8; FRAME_SIZE],
    pos: usize,
    status: FrameStatus,
}

impl FrameBuffer {
    pub fn new() -> Self {
        Self {
            data: [0; FRAME_SIZE],
            pos: 0,
            status: FrameStatus::Idle,
        }
    }

    pub fn get_byte(&mut self) -> Option<u8>{
        if self.status == FrameStatus::Running { 
            let val = self.data[self.pos];
            self.pos += 1;
            if self.pos == FRAME_SIZE { 
                self.pos = 0;
                self.status = FrameStatus::Idle;
            }
            return Some(val);
        } else {
            return None
        }
    }
}

static INCOMING_QUEUE: ISRQueue<u8, FRAME_SIZE> = ISRQueue::new();
static OUTGOING_QUEUE: ISRQueue<u8, FRAME_SIZE> = ISRQueue::new();
static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

enum SpiState {
    Init,
    Wait,
}

pub struct SlaveSPI<'a> {
    state: SpiState,
    incoming: isrqueue::Receiver<'a, u8, FRAME_SIZE>,
    in_frame: FrameBuffer,
    out_frame: FrameBuffer,
}

impl<'a> SlaveSPI<'a> {
    pub fn new(s: SPI) -> Self {
        // activate spi interrupt ( spie )
        // activate spi ( spe )
        // set slave (mstr = 0)
        s.spcr
            .write(|w| w.spie().set_bit().spe().set_bit().mstr().clear_bit());
        // Stash the SPI interface in a static
        avr_device::interrupt::free(|cs| {
            SPI_INT.borrow(cs).replace(Some(s));
        });
        Self {
            state: SpiState::Init,
            incoming: INCOMING_QUEUE.get_receiver(),
            in_frame: FrameBuffer::new(),
            out_frame: FrameBuffer::new(),
        }
    }

    pub async fn setup(&mut self) {
        poll_fn(|_cx| {
            match self.state {
                SpiState::Init => {
                    print!("Setup spi incoming");
                    // Set own task id
                    self.state = SpiState::Wait;
                    print!("Finished Setup");
                    Poll::Ready(())
                }
                SpiState::Wait => Poll::Ready(()),
            }
        })
        .await
    }

    pub async fn task(
        &mut self,
        mut com_incoming: channel::Receiver<'a, Command>,
        com_outgoing: channel::Sender<'a, Command>,
    ) {
        self.setup().await;
        loop {
            select_biased! {
                char_in = self.incoming.receive().fuse() => {
                    // Handle incoming char
                    if let Some(frame) = process_frame(char_in, &mut self.in_frame) {
                        match hubpack::deserialize::<Command>(&frame.data[3..FRAME_SIZE]) {
                            Ok((comm, _)) => {
                                com_outgoing.send(comm);
                            }
                            Err(_) => com_outgoing.send(Command::Fail),
                        }
                    }
                }
                comm = com_incoming.receive().fuse() => {
                    print!("incoming {:?}",comm);
                    self.outgoing_frame(comm)
                }
                complete => break
            }
        }
    }

    pub fn outgoing_frame(&mut self, value: Command) {

        // Prepare the outgoing frame
        let _ = hubpack::serialize(&mut self.out_frame.data[3..FRAME_SIZE], &value);
        self.out_frame.data[0] = SYNC1;
        self.out_frame.data[1] = SYNC2;
        self.out_frame.status = FrameStatus::Running;
        print!("{:?}",self.out_frame.data)
    }
}

pub fn process_frame(data: u8, pb: &mut FrameBuffer) -> Option<FrameBuffer> {
    // match up the packet data
    let val = match pb.pos {
        0 => {
            if data == SYNC1 {
                Ok(())
            } else {
                Err(())
            }
        } //Sync1
        1 => {
            if data == SYNC2 {
                Ok(())
            } else {
                Err(())
            }
        } //Sync2
        2 => Ok(()),  //Checksum
        3 => Ok(()),  //Command
        4 => Ok(()),  //data1
        5 => Ok(()),  //data2
        6 => Ok(()),  //data3
        7 => Ok(()),  //data4
        _ => Err(()), // everthing else
    };

    if val == Ok(()) {
        //  If the packet is good so far
        //serial_println!("{:?}",data).void_unwrap();
        pb.data[pb.pos] = data;
        pb.pos += 1;
        pb.status = FrameStatus::Running;
        // end of the frame
        if pb.pos == FRAME_SIZE {
            pb.pos = 0;
            pb.status = FrameStatus::Idle;
            //Packet Buffer Full ready to go
            return Some(pb.clone());
        }
        // Not Yet
        None
    } else {
        // bad packet data
        // reset
        pb.pos = 0;
        pb.status = FrameStatus::Idle;
        // not ready
        None
    }
}

#[avr_device::interrupt(atmega328p)]
fn SPI_STC() {
    avr_device::interrupt::free(|cs| {
        if let Some(spi) = &mut *SPI_INT.borrow(cs).borrow_mut() {
            let ch = spi.spdr.read().bits();
            INCOMING_QUEUE.send(ch);
        }
    });
}
