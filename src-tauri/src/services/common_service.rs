pub fn reg_mask(reg: u8) -> u8 {
    // レジスタを2ビットにマスク
    reg & 0b11
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reg_mask_zero() {
        assert_eq!(reg_mask(0b00), 0b00);
    }

    #[test]
    fn test_reg_mask_one() {
        assert_eq!(reg_mask(0b01), 0b01);
    }

    #[test]
    fn test_reg_mask_two() {
        assert_eq!(reg_mask(0b10), 0b10);
    }

    #[test]
    fn test_reg_mask_three() {
        assert_eq!(reg_mask(0b11), 0b11);
    }

    #[test]
    fn test_reg_mask_overflow() {
        // 8ビット入力で、下位2ビットだけを取得
        assert_eq!(reg_mask(0b11111111), 0b11);
    }

    #[test]
    fn test_reg_mask_higher_bits() {
        // 上位ビットは無視され、下位2ビットだけが保持される
        assert_eq!(reg_mask(0b10101010), 0b10);
    }

    #[test]
    fn test_reg_mask_alternating() {
        assert_eq!(reg_mask(0b01010101), 0b01);
    }
}
