/// A mutexed queue to spool up and provide structs
/// across tasks
/// majorly simplified version https://github.com/embassy-rs/embassy/blob/main/embassy-sync/src/channel.rs
///

/// Async executor for avr , data channel (CSP Hoare style)
/// Stolen from https://github.com/therustybits/zero-to-async
/// chapter 6 and converted.
use core::{
    cell::{Cell, RefCell},
    future::poll_fn,
    task::{Poll, Waker},
};

use avr_device::interrupt::Mutex;
use heapless::spsc::Queue as HQueue;

pub struct Queue<T, const N: usize> {
    items: Mutex<RefCell<HQueue<T, N>>>,
    waker: RefCell<Option<Waker>>,
}

impl<T, const N: usize> Queue<T, N> {
    pub fn new() -> Self {
        Self {
            items: Mutex::new(RefCell::new(HQueue::new())),
            waker: RefCell::new(None),
        }
    }

    pub fn get_sender(&self) -> Sender<T, N> {
        Sender { queue: &self }
    }

    pub fn get_receiver(&self) -> Receiver<T, N> {
        Receiver {
            queue: &self,
            state: ReceiverState::Init,
        }
    }

    fn send(&self, item: T) {
        // Open up the queue and add the item into it.
        avr_device::interrupt::free(|cs| {
            let items = &mut *self.items.borrow(cs).borrow_mut();
            if items.enqueue(item).is_ok() {
                if let Some(waker) = self.waker.borrow().as_ref() {
                    // Calling `wake()` consumes the waker, which means we'd have to
                    // `clone()` it first, so instead here we use `wake_by_ref()`
                    waker.wake_by_ref();
                }
            };
        });
    }

    fn receive(&self) -> Option<T> {
        crate::print!("get item");
        avr_device::interrupt::free(|cs| {
            crate::print!("in mutex");
            let items = &mut *self.items.borrow(cs).borrow_mut();
            items.dequeue()
        })
    }

    fn register(&self, waker: Waker) {
        self.waker.replace(Some(waker));
    }
}

pub struct Sender<'a, T, const N: usize> {
    queue: &'a Queue<T, N>,
}

impl<T, const N: usize> Sender<'_, T, N> {
    pub fn send(&self, item: T) {
        self.queue.send(item);
    }

    pub fn len(&self) -> usize {
        let mut len: usize = 0;
        avr_device::interrupt::free(|cs| {
            let items = &mut *self.queue.items.borrow(cs).borrow_mut();
            len =  items.len();
        });
        len
    }
}

enum ReceiverState {
    Init,
    Wait,
}

pub struct Receiver<'a, T, const N: usize> {
    queue: &'a Queue<T, N>,
    state: ReceiverState,
}

impl<T, const N: usize> Receiver<'_, T, N> {
    pub async fn receive(&mut self) -> T {
        poll_fn(|cx| match self.state {
            ReceiverState::Init => {
                self.queue.register(cx.waker().clone());
                self.state = ReceiverState::Wait;
                Poll::Pending
            }
            ReceiverState::Wait => match self.queue.receive() {
                Some(item) => Poll::Ready(item),
                None => Poll::Pending,
            },
        })
        .await
    }
}
