/// 4bit構造体
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct U4 {
    value: u8,
}

impl U4 {
    /// 4bitにマスクする
    pub fn mask(value: u8) -> Self {
        U4 {
            value: value & 0b1111,
        }
    }
    /// 内部値を取得する
    pub fn get(self) -> u8 {
        self.value
    }
}
