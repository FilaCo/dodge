use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bits(Vec<u8>);

impl Bits {
    const ENTRY_BITS: usize = u8::BITS as usize;

    pub fn new() -> Self {
        Self::with_len(1)
    }

    pub fn with_len(len: usize) -> Self {
        let packed_len = len.div_ceil(Self::ENTRY_BITS);

        Self(vec![0; packed_len])
    }

    pub fn len(&self) -> usize {
        self.0.len() * Self::ENTRY_BITS
    }

    pub fn get(&self, pos: usize) -> bool {
        self.0[pos] & Self::get_bit_pos(pos) != 0
    }

    pub fn set(&mut self, pos: usize) -> &mut Self {
        self.0[pos] |= Self::get_bit_pos(pos);

        self
    }

    pub fn unset(&mut self, pos: usize) -> &mut Self {
        self.0[pos] &= !Self::get_bit_pos(pos);

        self
    }

    pub fn toggle(&mut self, pos: usize) -> &mut Self {
        self.0[pos] ^= Self::get_bit_pos(pos);

        self
    }

    fn get_bit_pos(pos: usize) -> u8 {
        1u8 << (pos % Self::ENTRY_BITS)
    }
}

impl BitAnd for Bits {
    type Output = Bits;

    fn bitand(self, rhs: Bits) -> Self::Output {
        let diff_len = self.0.len().abs_diff(rhs.0.len());
        let (lesser, bigger) = if self.0.len() < rhs.0.len() {
            (self.0, rhs.0)
        } else {
            (rhs.0, self.0)
        };

        Self(
            lesser
                .iter()
                .enumerate()
                .map(|(idx, byte)| *byte & bigger[idx])
                .chain(vec![0; diff_len])
                .collect(),
        )
    }
}

impl BitAndAssign for Bits {
    fn bitand_assign(&mut self, rhs: Self) {
        let diff_len = self.0.len().abs_diff(rhs.0.len());

        if self.0.len() < rhs.0.len() {
            self.0.extend(vec![0; diff_len])
        }

        self.0.iter_mut().enumerate().for_each(|(idx, byte)| {
            *byte = if idx < rhs.0.len() {
                *byte & rhs.0[idx]
            } else {
                0
            }
        });
    }
}

impl BitOr for Bits {
    type Output = Bits;

    fn bitor(self, rhs: Bits) -> Self::Output {
        let (lesser, bigger) = if self.0.len() < rhs.0.len() {
            (self.0, rhs.0)
        } else {
            (rhs.0, self.0)
        };

        Self(
            lesser
                .iter()
                .enumerate()
                .map(|(idx, byte)| *byte | bigger[idx])
                .chain(bigger[lesser.len()..].iter().copied())
                .collect(),
        )
    }
}

impl BitOrAssign for Bits {
    fn bitor_assign(&mut self, rhs: Self) {
        if self.0.len() < rhs.0.len() {
            self.0.extend(rhs.0[self.0.len()..].iter());
        }

        self.0.iter_mut().enumerate().for_each(|(idx, byte)| {
            if idx < rhs.0.len() {
                *byte |= rhs.0[idx];
            }
        });
    }
}

impl BitXor for Bits {
    type Output = Bits;

    fn bitxor(self, rhs: Self) -> Self::Output {
        let (lesser, bigger) = if self.0.len() < rhs.0.len() {
            (self.0, rhs.0)
        } else {
            (rhs.0, self.0)
        };

        Self(
            lesser
                .iter()
                .enumerate()
                .map(|(idx, byte)| *byte ^ bigger[idx])
                .chain(bigger[lesser.len()..].iter().copied())
                .collect(),
        )
    }
}

impl BitXorAssign for Bits {
    fn bitxor_assign(&mut self, rhs: Self) {
        if self.0.len() < rhs.0.len() {
            self.0.extend(rhs.0[self.0.len()..].iter());
        }

        self.0.iter_mut().enumerate().for_each(|(idx, byte)| {
            if idx < rhs.0.len() {
                *byte ^= rhs.0[idx];
            }
        });
    }
}

impl Shr<usize> for Bits {
    type Output = Bits;

    fn shr(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

impl ShrAssign<usize> for Bits {
    fn shr_assign(&mut self, rhs: usize) {
        todo!()
    }
}

impl Shl<usize> for Bits {
    type Output = Bits;

    fn shl(self, rhs: usize) -> Self::Output {
        todo!()
    }
}

impl ShlAssign<usize> for Bits {
    fn shl_assign(&mut self, rhs: usize) {
        todo!()
    }
}

impl From<u128> for Bits {
    fn from(value: u128) -> Self {
        Self(value.to_le_bytes().to_vec())
    }
}

impl From<u64> for Bits {
    fn from(value: u64) -> Self {
        Self(value.to_le_bytes().to_vec())
    }
}

impl From<usize> for Bits {
    fn from(value: usize) -> Self {
        Self(value.to_le_bytes().to_vec())
    }
}

impl From<u32> for Bits {
    fn from(value: u32) -> Self {
        Self(value.to_le_bytes().to_vec())
    }
}

impl From<u16> for Bits {
    fn from(value: u16) -> Self {
        Self(value.to_le_bytes().to_vec())
    }
}

impl From<u8> for Bits {
    fn from(value: u8) -> Self {
        Self(vec![value])
    }
}

impl From<Vec<u8>> for Bits {
    fn from(value: Vec<u8>) -> Self {
        assert!(!value.is_empty());

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
