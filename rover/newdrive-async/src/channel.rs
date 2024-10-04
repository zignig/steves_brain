/// Async executor for avr , data channel (CSP Hoare style)
/// Stolen from https://github.com/therustybits/zero-to-async
/// chapter 6 and converted. 
/// This provides a single item channel to move data and events 
/// between tasks.

// Multiple senders works , but the reciver consumes the events.

use core::{
    cell::{Cell, RefCell},
    future::poll_fn,
    task::{Poll, Waker},
};


// This is for notifiny another task that there
// are things todo
pub struct Channel<T> {
    item: Cell<Option<T>>,
    waker: RefCell<Option<Waker>>,
}

impl<T> Channel<T> {

    pub fn new() -> Self {
        Self {
            item: Cell::new(None),
            waker: RefCell::new(None),
        }
    }

    pub fn get_sender(&self) -> Sender<T> {
        Sender { channel: &self }
    }

    pub fn get_receiver(&self) -> Receiver<T> {
        Receiver {
            channel: &self,
            state: ReceiverState::Init,
        }
    }

    fn send(&self, item: T) {
        self.item.replace(Some(item));
        if let Some(waker) = self.waker.borrow().as_ref() {
            // Calling `wake()` consumes the waker, which means we'd have to
            // `clone()` it first, so instead here we use `wake_by_ref()`
            waker.wake_by_ref();
        }
    }

    fn receive(&self) -> Option<T> {
        self.item.take()
    }

    fn register(&self, waker: Waker) {
        self.waker.replace(Some(waker));
    }
}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
}

impl<T> Sender<'_, T> {
    pub fn send(&self, item: T) {
        self.channel.send(item);
    }
}

// Important to remember is that this will consume
// multiple senders one reciver
enum ReceiverState {
    Init,
    Wait,
}

// Extract a reciver, this will wake a task.
pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    state: ReceiverState,
}

impl<T> Receiver<'_, T> {
    pub async fn receive(&mut self) -> T {
        poll_fn(|cx| match self.state {
            ReceiverState::Init => {
                self.channel.register(cx.waker().clone());
                self.state = ReceiverState::Wait;
                Poll::Pending
            }
            ReceiverState::Wait => match self.channel.receive() {
                Some(item) => Poll::Ready(item),
                None => Poll::Pending,
            }
        })
        .await
    }
}
