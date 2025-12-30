use crate::models::bit::U2;

/// CPU構造体
#[derive(Debug)]
pub struct Cpu {
    /// プログラムカウンタ(2bit)
    pub pc_counter: U2,
    /// 汎用レジスタA(2bit)
    pub a_reg: U2,
    /// キャリーフラグ(1bit)
    pub c_flag: bool,
}

impl Cpu {
    /// Cpu状態をリセットする関数
    /// # Returns
    /// * リセットされたCpu構造体
    pub fn reset() -> Self {
        Self::default()
    }
}

impl Default for Cpu {
    /// デフォルトのCPU状態を返す
    /// # Returns
    /// * デフォルト状態のCpu構造体
    fn default() -> Self {
        Cpu {
            // プログラムカウンタを0に初期化
            pc_counter: U2::mask(0),
            // 汎用レジスタAを0に初期化
            a_reg: U2::mask(0),
            // キャリーフラグをfalseに初期化
            c_flag: false,
        }
    }
}
