/// 2bit構造体
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct U2 {
    value: u8,
}

impl U2 {
    /// 2bitにマスクする
    pub fn mask(value: u8) -> Self {
        U2 {
            value: value & 0b11,
        }
    }
    /// 内部値を取得する
    pub fn get(self) -> u8 {
        self.value
    }
}
