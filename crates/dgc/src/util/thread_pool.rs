#[derive(Debug)]
pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(size: ThreadPoolSize) -> Self {
        todo!()
    }

    pub fn execute<F>(&self, f: F) {
        todo!()
    }
}

impl Default for ThreadPool {
    fn default() -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ThreadPoolSize(u32);
