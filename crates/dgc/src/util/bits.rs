use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bits(Vec<u8>);

impl Bits {
    pub fn new() -> Self {
        Self::with_capacity(1)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity((capacity / size_of::<u8>()).max(1)))
    }

    pub fn capacity(&self) -> usize {
        self.0.capacity() * size_of::<u8>()
    }

    pub fn len(&self) -> usize {
        self.0.len() * size_of::<u8>()
    }
}

impl BitAnd<Bits> for Bits {
    type Output = Bits;

    fn bitand(self, rhs: Bits) -> Self::Output {
        todo!()
    }
}

impl BitAndAssign for Bits {
    fn bitand_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl BitOr<Bits> for Bits {
    type Output = Bits;

    fn bitor(self, rhs: Bits) -> Self::Output {
        todo!()
    }
}

impl BitOrAssign for Bits {
    fn bitor_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl From<u64> for Bits {
    fn from(value: u64) -> Self {
        todo!()
    }
}

impl From<Vec<u8>> for Bits {
    fn from(value: Vec<u8>) -> Self {
        assert!(value.len() > 0);

        Self(value)
    }
}

impl Default for Bits {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
