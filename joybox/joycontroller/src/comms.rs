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
use core::cell::RefCell;
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
    Start,
    Inside,
    Finished,
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
            status: FrameStatus::Finished,
        }
    }
}

// TODO use phantom data to consume the pins
pub struct SlaveSPI {}

pub struct DataComms {
    in_frame: FrameBuffer,
    in_comm: Ring<Command,RING_SIZE>,
    out_frame: FrameBuffer,
    out_comm: Ring<FrameBuffer,RING_SIZE>
}

impl DataComms { 
    pub fn new() -> Self { 
        Self { 
            in_frame: FrameBuffer::new(),
            in_comm: Ring::<Command, RING_SIZE>::new(),
            out_frame: FrameBuffer::new(),
            out_comm: Ring::<FrameBuffer, RING_SIZE>::new()
        }
    }
}

static COMMS: Mutex<RefCell<Option<DataComms>>> = Mutex::new(RefCell::new(None));

// Incoming data

// guarded access to the SPI object
static SPI_INT: Mutex<RefCell<Option<SPI>>> = Mutex::new(RefCell::new(None));

// guarded incoming data PacketBuffer
static DATA_FRAME: Mutex<RefCell<Option<FrameBuffer>>> = Mutex::new(RefCell::new(None));

// ring buffer for the created commands
static COMMAND_RING: Mutex<RefCell<Option<Ring<Command, RING_SIZE>>>> =
    Mutex::new(RefCell::new(None));

// Outgoing data

// guarded outgoing data PacketBuffer
static OUT_FRAME: Mutex<RefCell<Option<FrameBuffer>>> = Mutex::new(RefCell::new(None));

// ring buffer for outgoing packets
static OUT_RING: Mutex<RefCell<Option<Ring<FrameBuffer, RING_SIZE>>>> =
    Mutex::new(RefCell::new(None));

impl SlaveSPI {
    pub fn init(s: SPI) {
        // activate spi interrupt ( spie )
        // activate spi ( spe )
        // set slave (mstr = 0)
        s.spcr
            .write(|w| w.spie().set_bit().spe().set_bit().mstr().clear_bit());
        // put the spi , ring buffer and packetbuffer  into a protected globals
        avr_device::interrupt::free(|cs| {
            // data incoming
            COMMS.borrow(cs).replace(Some(DataComms::new()));
            SPI_INT.borrow(cs).replace(Some(s));

            // Old contstruct
            DATA_FRAME.borrow(cs).replace(Some(FrameBuffer::new()));
            COMMAND_RING
                .borrow(cs)
                .replace(Some(Ring::<Command, RING_SIZE>::new()));
            // data outgoing
            OUT_FRAME.borrow(cs).replace(Some(FrameBuffer::new()));
            OUT_RING
                .borrow(cs)
                .replace(Some(Ring::<FrameBuffer, RING_SIZE>::new()));
        });
    }
}

#[avr_device::interrupt(atmega328p)]
fn SPI_STC() {
    avr_device::interrupt::free(|cs| {
        // Incoming Data
        let mut flag: bool = false;
        let mut data: u8 = 0;

        // get the data byte from the SPI bus
        if let Some(s) = &mut *SPI_INT.borrow(cs).borrow_mut() {

            //let val = s.spsr.read().spif().bit();
            data = s.spdr.read().bits();
            while (s.spsr.read().spif().bit_is_set()) {
                serial_println!("doot!");
            }
            //serial_println!("{:#?} --> {:#?}",val,data);
        }
        //serial_println!("{}",data);
        // put the data into the buffer
        if let Some(pb) = &mut *DATA_FRAME.borrow(cs).borrow_mut() {
            // push the byte into the packet checker
            if let Some(the_packet) = process_packet(data, pb) {
                // the packet is well formed
                //const SIZE : usize = Command::MAX_SIZE;
                //serial_println!("{:#?}", the_packet.data[3..FRAME_SIZE]);
                // deserialize the command part of the packet
                let (comm, _) =
                    hubpack::deserialize::<Command>(&the_packet.data[3..FRAME_SIZE]).unwrap();
                // chuck the command into a ring buffer
                //serial_println!("extra = {:#?}",extra);
                //serial_println!("command = {:#?}",comm);
                if let Some(cr) = &mut *COMMAND_RING.borrow(cs).borrow_mut() {
                    serial_println!("{:?}",cr.size());
                    cr.append(comm);
                }
                flag = true;
            }
        }

        // // Outgoing data
        // // When the interface is ready , spool out a frame
        // // serialize into the SPI interface.

        // if let Some(pb) = &mut *OUT_FRAME.borrow(cs).borrow_mut() {
        //     if let Some(s) = &mut *SPI_INT.borrow(cs).borrow_mut() {
        //         let val = s.spsr.read().spif().bit();
        //         //serial_println!("--> {:#?}",val);
        //         //serial_println!("{:#?}", pb.data[pb.pos]);
        //         //serial_println!("{:#?}",s.spsr.read().bits());
        //         pb.pos += 1;
        //         // get new frame
        //         if (pb.pos == FRAME_SIZE) || flag {
        //             // get new frame
        //             if let Some(frame) = fetch_frame() {
        //                 pb.data = frame.data;
        //                 serial_println!("out frame: {:?}", frame.data);
        //                 // Load the first byte
        //                 //     pb.pos = 0;
        //             } else {
        //                 *pb = FrameBuffer::new();
        //                 serial_println!("empty frame");
        //             }
        //             pb.pos = 0;
        //             // pb.pos = 0;
        //             // pb.data[0] = SYNC1;
        //             // pb.data[1] = SYNC2;
        //             // hubpack::serialize(&mut pb.data[3..FRAME_SIZE], &Command::GetMillis(1234356));
        //             // serial_println!("{:#?}", pb.data[..]);

        //             flag = false;
        //         }
        //         //let val = s.spsr.read().wcol().bit();
        //         // wait for the transaction to finish
        //         //serial_println!("{:#?} --> {:#?}",val,pb.data[pb.pos]);
        //         unsafe {
        //             s.spdr.write(|w| w.bits(pb.data[pb.pos]));
        //             //s.spdr.write(|w| w.bits(pb.pos as u8));
        //         }
        //     }
        // }
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
        pb.status = FrameStatus::Inside;
        // end of the frame
        if pb.pos == FRAME_SIZE {
            pb.pos = 0;
            pb.status = FrameStatus::Finished;
            //Packet Buffer Full ready to go
            return Some(pb.clone());
        }
        // Not Yet
        None
    } else {
        // bad packet data
        // reset
        pb.pos = 0;
        pb.status = FrameStatus::Start;
        // not ready
        None
    }
}

//Fetch a command out of the ring buffer
pub fn fetch_command() -> Option<Command> {
    let mut comm = None;
    avr_device::interrupt::free(|cs| {
        if let Some(cr) = &mut *COMMAND_RING.borrow(cs).borrow_mut() {
            //serial_println!("len: {} ",cr.len()).void_unwrap();
            if let Some(val) = cr.pop() {
                //serial_println!("value: {:#?} ",val).void_unwrap();
                comm = Some(val);
            }
        }
    });
    comm
}

fn fetch_frame() -> Option<FrameBuffer> {
    let mut frame = None;
    avr_device::interrupt::free(|cs| {
        if let Some(cr) = &mut *OUT_RING.borrow(cs).borrow_mut() {
            //serial_println!("len: {} ",cr.len()).void_unwrap();
            if let Some(val) = cr.pop() {
                //serial_println!("value: {:#?} ",val).void_unwrap();
                // get new frame
                // serial_println!("{:?}", val.data[..]);

                frame = Some(val);
            }
        }
    });
    frame
}

pub fn send_command(comm: Command) {
    //serial_println!("{:#?}", comm);
    avr_device::interrupt::free(|cs| {
        if let Some(cr) = &mut *OUT_RING.borrow(cs).borrow_mut() {
            let mut pb = FrameBuffer::new();
            pb.pos = 0;
            pb.data[0] = SYNC1;
            pb.data[1] = SYNC2;
            hubpack::serialize(&mut pb.data[3..FRAME_SIZE], &comm);

            //serial_println!("{:?}", pb.data[..]);

            // // if the ring is empty put it straight into the buffer
            // if cr.is_empty() {
            //     if let Some(out_frame) = &mut *OUT_FRAME.borrow(cs).borrow_mut() {
            //         out_frame.data = pb.data;
            //     }
            //     return;
            // }
            cr.append(pb);
        }
    });
}
