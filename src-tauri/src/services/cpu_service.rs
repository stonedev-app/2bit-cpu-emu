use crate::utils::mask::mask_2bit;

/// プログラムカウンター（PC）をインクリメントする関数
/// # Arguments
/// * `pc` - 現在のプログラムカウンターの値
/// # Returns
/// * インクリメントされたプログラムカウンターの値（2ビットにマスク済み）
pub fn inc_pc(pc: u8) -> u8 {
    mask_2bit(pc + 1)
}

/// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    /// インクリメントでPCカウンターが正しく更新されることを確認するテスト
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
}
