/// A simple 4-bit unsigned integer type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct U4 {
    value: u8,
}

impl U4 {
    /// Masks the input value to fit into 4 bits.
    pub fn mask(value: u8) -> Self {
        U4 {
            value: value & 0b1111,
        }
    }
    /// Retrieves the underlying u8 value.
    pub fn get(self) -> u8 {
        self.value
    }
}
