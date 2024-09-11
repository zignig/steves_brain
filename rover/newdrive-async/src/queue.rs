/// A mutexed queue to spool up and provide structs
/// across tasks
/// majorly simplified version https://github.com/embassy-rs/embassy/blob/main/embassy-sync/src/channel.rs
///

/// Async executor for avr , data channel (CSP Hoare style)
/// Stolen from https://github.com/therustybits/zero-to-async
/// chapter 6 and converted.
use core::{
    borrow::BorrowMut, cell::{Cell, RefCell}, future::poll_fn, task::{Poll, Waker}
};

use avr_device::interrupt::Mutex;
use heapless::Deque;

// Inner struct

pub struct QueueState< T , const N: usize> { 
    queue: Deque<T,N>,
    waker: RefCell<Option<Waker>>
}

impl <T, const N: usize> QueueState<T,N>{ 
    const fn new() -> Self { 
        QueueState {
            queue: Deque::new(),
            waker: RefCell::new(None)
        }
    }
}


// Wrapping Struct , use this. 
// Is mutexed so it can be used from an ISR
pub struct Queue<T, const N: usize> {
    inner: Mutex<RefCell<QueueState<T,N>>>
}

impl<T, const N: usize> Queue<T, N> {
    pub const fn new() -> Self {
        Self {
            inner: Mutex::new(RefCell::new(QueueState::new()))
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

    pub fn send(&self, item: T) {
        // Open up the queue and add the item into it.
        avr_device::interrupt::free(|cs| {
            let inner = &mut *self.inner.borrow(cs).borrow_mut();
            if inner.queue.push_back(item).is_ok() {
                if let Some(waker) = &mut *inner.waker.borrow_mut(){
                    // Calling `wake()` consumes the waker, which means we'd have to
                    // `clone()` it first, so instead here we use `wake_by_ref()`
                    waker.wake_by_ref();
                }
            };
        });
    }

    fn receive(&self) -> Option<T> {
        avr_device::interrupt::free(|cs| {
            let inner = &mut *self.inner.borrow(cs).borrow_mut();
            inner.queue.pop_front()
        })
    }

    fn register(&self, waker: Waker) {
        avr_device::interrupt::free(|cs| {
            let inner = &mut *self.inner.borrow(cs).borrow_mut();
            inner.waker.replace(Some(waker));
        });
    }

    pub fn len(&self) -> usize { 
        avr_device::interrupt::free(|cs| {
            let inner = &mut *self.inner.borrow(cs).borrow_mut();
            inner.queue.len()
        })
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
        self.queue.len()
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
