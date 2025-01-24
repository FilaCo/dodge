#[derive(Debug)]
pub struct ThreadsNumber(u8);

impl ThreadsNumber {
    const MIN_THREADS_NUMBER: u8 = 1;
    const DEFAULT_THREADS_NUMBER: u8 = 4;
    pub fn new() -> Self {
        Self(Self::DEFAULT_THREADS_NUMBER)
    }

    pub fn min() -> Self {
        Self(Self::MIN_THREADS_NUMBER)
    }
}

impl Default for ThreadsNumber {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u8> for ThreadsNumber {
    fn from(value: u8) -> Self {
        assert!(value >= Self::MIN_THREADS_NUMBER);

        Self(value)
    }
}
