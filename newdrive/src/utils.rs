// Some util functions 

//! Liberated from https://github.com/kchmck/moving_avg.rs
//! 
//! Moving average filter.


/// Computes a moving average over a ring buffer of numbers.
#[derive(Clone,Copy)]
pub struct MovingAverage16 {
    /// Current history of numbers.
    hist: [i16;16],
    /// Size of the history, as T.
    size: usize,
    /// Index in the history vector to replace next.
    pos: usize,
}

impl MovingAverage16 {
    /// Create a new `MovingAverage` that averages over the given amount of numbers.
    pub fn new() -> Self {
        let  hist: [i16;16] = [0;16];
        MovingAverage16 {
            hist: hist,
            size: 16,
            pos: 0,
        }
    }

    /// Add the given number to the history, overwriting the oldest number, and return the
    /// resulting moving average.
    pub fn feed(&mut self, num: i16) -> i16 {
        self.hist[self.pos] = num;
        self.pos += 1;
        self.pos %= 16;
        self.avg()
    }

    /// Calculate moving average based on the current history.
    fn avg(&self) -> i16 {
        self.hist.iter().fold(0, |s, &x| s + x) / (self.size as i16)
    }
}