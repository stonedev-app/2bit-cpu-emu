/// レジスタを2ビットにマスクする関数
/// 例えば、入力が0b100の場合、出力は0b00になる。
/// これは、2ビットCPUのレジスタが0から3までの範囲で動作することを保証するために使用される。
/// # Arguments
/// * `reg` - マスクする8ビットのレジスタ値
/// # Returns
/// * マスクされた2ビットのレジスタ値
pub fn reg_mask(reg: u8) -> u8 {
    // レジスタを2ビットにマスク
    reg & 0b11
}

/// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    /// 0b00がそのまま0b00になることを確認するテスト
    #[test]
    fn test_reg_mask_zero() {
        assert_eq!(reg_mask(0b00), 0b00);
    }

    /// 0b01がそのまま0b01になることを確認するテスト
    #[test]
    fn test_reg_mask_one() {
        assert_eq!(reg_mask(0b01), 0b01);
    }

    /// 0b10がそのまま0b10になることを確認するテスト
    #[test]
    fn test_reg_mask_two() {
        assert_eq!(reg_mask(0b10), 0b10);
    }

    /// 0b11がそのまま0b11になることを確認するテスト
    #[test]
    fn test_reg_mask_three() {
        assert_eq!(reg_mask(0b11), 0b11);
    }

    /// 0b11111111が0b11になることを確認するテスト
    #[test]
    fn test_reg_mask_overflow() {
        assert_eq!(reg_mask(0b11111111), 0b11);
    }

    /// 0b10101010が0b10になることを確認するテスト
    #[test]
    fn test_reg_mask_higher_bits() {
        // 上位ビットは無視され、下位2ビットだけが保持される
        assert_eq!(reg_mask(0b10101010), 0b10);
    }

    /// 0b01010101が0b01になることを確認するテスト
    #[test]
    fn test_reg_mask_alternating() {
        assert_eq!(reg_mask(0b01010101), 0b01);
    }
}
