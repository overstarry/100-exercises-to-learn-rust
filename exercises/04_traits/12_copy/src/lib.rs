// TODO: implement the necessary traits to make the test compile and pass.
//  You *can't* modify the test.

#[derive(Debug,Clone,Copy,PartialEq)]
pub struct WrappingU32 {
    value: u32,
}

impl WrappingU32 {
    pub fn new(value: u32) -> Self {
        Self { value }
    }
}

use std::ops::Add;

impl Add for WrappingU32 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            value: self.value.wrapping_add(other.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ops() {
        let x = WrappingU32::new(42);
        let y = WrappingU32::new(31);
        let z = WrappingU32::new(u32::MAX);
        assert_eq!(x + y + y + z, WrappingU32::new(103));
    }
}
