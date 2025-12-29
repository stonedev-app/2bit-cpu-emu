use crate::services::common_service::reg_mask;

pub fn inc_pc(pc: u8) -> u8 {
    reg_mask(pc + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inc_pc_from_zero() {
        assert_eq!(inc_pc(0), 1);
    }

    #[test]
    fn test_inc_pc_from_one() {
        assert_eq!(inc_pc(1), 2);
    }

    #[test]
    fn test_inc_pc_from_two() {
        assert_eq!(inc_pc(2), 3);
    }

    #[test]
    fn test_inc_pc_from_three_wraps_to_zero() {
        // 3 + 1 = 4 (0b100), その後 reg_mask で 0b00 = 0 になる
        assert_eq!(inc_pc(3), 0);
    }

    #[test]
    fn test_inc_pc_sequence() {
        // PCカウンターの正常なシーケンス: 0 -> 1 -> 2 -> 3 -> 0 -> ...
        let mut pc = 0;
        pc = inc_pc(pc);
        assert_eq!(pc, 1);
        pc = inc_pc(pc);
        assert_eq!(pc, 2);
        pc = inc_pc(pc);
        assert_eq!(pc, 3);
        pc = inc_pc(pc);
        assert_eq!(pc, 0);
    }

    #[test]
    fn test_inc_pc_at_max_valid_value() {
        // 最大の有効値（3）からインクリメント
        // これは wrap-around の確認
        assert_eq!(inc_pc(3), 0);
    }
}
