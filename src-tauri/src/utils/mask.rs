/// 2ビットにマスクする関数
/// # Arguments
/// * `value` - マスクする8ビットの値
/// # Returns
/// * マスクされた2ビットの値
pub fn mask_2bit(value: u8) -> u8 {
    // 2ビットにマスク
    value & 0b11
}

/// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    /// 0b01がそのまま0b01になることを確認するテスト
    #[test]
    fn test_value_mask_one() {
        assert_eq!(mask_2bit(0b01), 0b01);
    }

    /// 0b10がそのまま0b10になることを確認するテスト
    #[test]
    fn test_value_mask_two() {
        assert_eq!(mask_2bit(0b10), 0b10);
    }

    /// 0b11111111が0b11になることを確認するテスト
    #[test]
    fn test_value_mask_overflow() {
        assert_eq!(mask_2bit(0b11111111), 0b11);
    }
}
