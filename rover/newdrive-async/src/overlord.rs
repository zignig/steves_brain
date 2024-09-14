use futures::Future;

/// This is the main scheduler and reflex generator
/// becuase these are async tasks they need to be decoupled
/// This means that there needs to be some channels an queues talk to stuff
/// This thing need to watch all the things and do stuff.


enum SystemState { 
    Init,
    Setup,
    Running,
    Idle,
    Shutdown,
    Error,
    Fatal
}

pub struct OverLord { 
    state: SystemState
}

impl OverLord { 
    pub fn new() -> Self { 
        Self { 
            state: SystemState::Init
        }
    }
}

impl Future for OverLord {
    type Output = ();

    fn poll(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> core::task::Poll<Self::Output> {
        todo!()
    } 
}