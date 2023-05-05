//! Comms interface
//! SPI slave interface that makes packets
//! lifted from <https://medium.com/embedism/the-definitive-guide-on-writing-a-spi-communications-protocol-for-stm32-73594add4c09>
//! and rewritten again in rust ( after c++ )
//!
//! This is a frame based slave SPI interface
//!

use crate::serial_println;
use hubpack;

use crate::commands::Command;
use crate::ring_buffer::Ring;
use arduino_hal::pac::SPI;
use avr_device;

use avr_device::interrupt::Mutex;
//use core::borrow::BorrowMut;
use core::cell::{Cell, RefCell};

use core::u8;

pub const FRAME_SIZE: usize = 8;
/// Frame Header
pub const SYNC1: u8 = 0xF;
pub const SYNC2: u8 = 0xE;

// Size of the ring buffer.
pub const RING_SIZE: usize = 8;

// Need to structure the outgoing
// status of the frame
#[derive(Clone, Copy)]
pub enum FrameStatus {
    Running,
    Idle,
}

pub trait Buffer {}

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
            status: FrameStatus::Finished,
        }
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self {
            data: [0; FRAME_SIZE],
            pos: 0,
            status: FrameStatus::Idle,
        }
    }
}

// TODO use phantom data to consume the pins
pub struct SlaveSPI {}

pub struct DataComms {
    counter: u8,
    in_frame: FrameBuffer,
    in_comm: Ring<Command, RING_SIZE>,
    out_data: u8,
    out_frame: FrameBuffer,
    out_comm: Ring<FrameBuffer, RING_SIZE>,
}

impl DataComms {
    pub fn new() -> Self {
        Self {
            counter: 0,
            in_frame: FrameBuffer::new(),
            in_comm: Ring::<Command, RING_SIZE>::new(),
            out_data: 0,
            out_frame: FrameBuffer::new(),
            out_comm: Ring::<FrameBuffer, RING_SIZE>::new(),
        }
    }
}

static COMMS: Mutex<RefCell<Option<DataComms>>> = Mutex::new(RefCell::new(None));

// guarded access to the SPI object
static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));
static COUNTER: Mutex<Cell<u8>> = Mutex::new(Cell::new(0));

impl SlaveSPI {
    pub fn init(s: SPI) {
        // activate spi interrupt ( spie )
        // activate spi ( spe )
        // set slave (mstr = 0)
        s.spcr
            .write(|w| w.spie().set_bit().spe().set_bit().mstr().clear_bit());
        avr_device::interrupt::free(|cs| {
            COMMS.borrow(cs).replace(Some(DataComms::new()));
            SPI_INT.borrow(cs).replace(Some(s));
            //COUNTER.borrow(cs).replace(Some(0 as u8));
        });
    }
}

// //#[avr_device::interrupt(atmega328p)]
// fn test_SPI_STC() {
//     avr_device::interrupt::free(|cs| {
//         // Incoming Data
//         //let mut flag: bool = false;
//         let mut data: u8 = 0;
//         if let Some(s) = &mut *SPI_INT.borrow(cs).borrow_mut() {
//             data = s.spdr.read().bits();
//             let counter = COUNTER.borrow(cs);
//             unsafe {
//                 s.spdr.write(|w| w.bits(counter.get()));
//                 //s.spdr.write(|w| w.bits(pb.pos as u8));
//             }
//             counter.set((counter.get() + 1) % 255);
//             //serial_println!("{:#?}",s.spsr.read().wcol().bit_is_set());
//         }
//     });
// }

#[avr_device::interrupt(atmega328p)]
fn SPI_STC() {
    avr_device::interrupt::free(|cs| {
        // Incoming Data
        let mut flag: bool = false;
        let mut data: u8 = 0;

        // Open the comms struct.
        if let Some(comm_data) = &mut *COMMS.borrow(cs).borrow_mut() {
            // get the data byte from the SPI bus and put a new byte in.
            if let Some(s) = &mut *SPI_INT.borrow(cs).borrow_mut() {
                data = s.spdr.read().bits();
                // write the out going data
                unsafe {
                    s.spdr.write(|w| w.bits(comm_data.data));
                }
            }

            // push the incoming byte into the packet checker
            if let Some(the_packet) = process_packet(data, &mut comm_data.in_frame) {
                // the_packet is good
                let (comm, _) =
                    hubpack::deserialize::<Command>(&the_packet.data[3..FRAME_SIZE]).unwrap();
                //serial_println!("{:?}",the_packet.data[..]);
                comm_data.in_comm.append(comm);
                flag = true;
            }

            // Outgoing Frames
            broken
            // Outgoing data
            // When the interface is ready , spool out a frame
            // serialize into the SPI interface.
            // if (comm_data.out_frame.pos == FRAME_SIZE) || flag {
            //     // get new frame
            //     if let Some(frame) = fetch_frame() {
            //         comm_data.out_frame.data = frame.data;
            //         serial_println!("out frame: {:?}", frame.data);
            //     } else {
            //         comm_data.out_frame = FrameBuffer::new();
            //         serial_println!("empty frame");
            //     }
            //     comm_data.out_frame.pos = 0;
            //     flag = false;
            //     comm_data.data = comm_data.out_frame.data[comm_data.out_frame.pos];
            // }

            // increment the frame pos and load the outgoing dat for the _next_ write / read
            comm_data.out_data = comm_data.out_frame.data[com_data.out_frame.pos];
            comm_data.out_frame.pos += 1;
        }
    });
}

#[inline(always)]
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

//Fetch a command out of the ring buffer
pub fn fetch_command() -> Option<Command> {
    let mut comm = None;
    avr_device::interrupt::free(|cs| {
        if let Some(cd) = &mut *COMMS.borrow(cs).borrow_mut() {
            //serial_println!("len: {} ",cr.len()).void_unwrap();
            if let Some(val) = cd.in_comm.pop() {
                //serial_println!("value: {:#?} ",val).void_unwrap();
                comm = Some(val);
            }
        }
    });
    comm
}

// Fetch a frame from the ring out
fn fetch_frame() -> Option<FrameBuffer> {
    let mut frame = None;
    avr_device::interrupt::free(|cs| {
        if let Some(comm_data) = &mut *COMMS.borrow(cs).borrow_mut() {
            if let Some(val) = comm_data.out_comm.pop() {
                frame = Some(val);
            }
        }
    });
    frame
}

pub fn send_command(comm: Command) {
    //serial_println!("{:#?}", comm);
    avr_device::interrupt::free(|cs| {
        if let Some(cd) = &mut *COMMS.borrow(cs).borrow_mut() {
            let mut pb = FrameBuffer::new();
            pb.pos = 0;
            pb.data[0] = SYNC1;
            pb.data[1] = SYNC2;
            hubpack::serialize(&mut pb.data[3..FRAME_SIZE], &comm);
            // CHECK IF THE INCOMING BUFFER IS BUSY.
            if cd.in_frame.status == FrameStatus::Idle {
                // and there are no waiting packets
                if cd.out_comm.is_empty() {
                    // Put the frame straight into the outgoing buffer.
                    //serial_println!("{:?}", pb.data[..]);
                    // // if the ring is empty put it straight into the buffer
                    // if cr.is_empty() {
                    //     if let Some(out_frame) = &mut *OUT_FRAME.borrow(cs).borrow_mut() {
                    //         out_frame.data = pb.data;
                    //     }
                    //     return;
                    // }
                }
            } else {
                // Chuck it into the ring buffer
                cd.out_comm.append(pb);
            }


        }
    });
}
