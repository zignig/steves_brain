use arduino_hal::pac::SPI;
use avr_device::interrupt::Mutex;
use core::{cell::RefCell, future::poll_fn, task::Poll};
use ufmt::derive::uDebug;
use hubpack;
use hubpack::SerializedSize;

use crate::{
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
}

static INCOMING_QUEUE: ISRQueue<u8, 8> = ISRQueue::new();
static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

enum SpiState {
    Init,
    Wait,
}

pub struct SlaveSPI<'a> {
    state: SpiState,
    incoming: isrqueue::Receiver<'a, u8, 8>,
    outgoing: Queue<Command, 8>,
    frame: FrameBuffer,
}

impl<'a> SlaveSPI<'a> {
    pub fn new(s: SPI) -> Self {
        // activate spi interrupt ( spie )
        // activate spi ( spe )
        // set slave (mstr = 0)
        s.spcr
            .write(|w| w.spie().set_bit().spe().set_bit().mstr().clear_bit());
        avr_device::interrupt::free(|cs| {
            SPI_INT.borrow(cs).replace(Some(s));
        });
        Self {
            state: SpiState::Init,
            incoming: INCOMING_QUEUE.get_receiver(),
            outgoing: Queue::new(),
            frame: FrameBuffer::new(),
        }
    }

    pub async fn setup(&mut self) {
        poll_fn(|cx| {
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

    pub async fn task(&mut self) {
        self.setup().await;
        loop {
            let val = self.incoming.receive().await;
            if let Some(frame) = process_packet(val, &mut self.frame) {
                let (comm, _) =
                    hubpack::deserialize::<Command>(&frame.data[3..FRAME_SIZE]).unwrap();
                crate::print!("{:?}",comm);
            }
        }
    }
}

pub fn process_packet(data: u8, pb: &mut FrameBuffer) -> Option<FrameBuffer> {
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
