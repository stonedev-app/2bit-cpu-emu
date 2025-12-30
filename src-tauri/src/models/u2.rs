/// A simple 2-bit unsigned integer type.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct U2 {
    value: u8,
}

impl U2 {
    /// Masks the input value to fit into 2 bits.
    pub fn mask(value: u8) -> Self {
        U2 {
            value: value & 0b11,
        }
    }
    /// Retrieves the underlying u8 value.
    pub fn get(self) -> u8 {
        self.value
    }
}
