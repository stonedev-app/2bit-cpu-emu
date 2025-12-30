/// CPU構造体
pub struct Cpu {
    /// プログラムカウンタ(2bit)
    pub pc_counter: u8,
    /// 汎用レジスタA(2bit)
    pub a_reg: u8,
    /// キャリーフラグ(1bit)
    pub c_flag: bool,
}

impl Cpu {
    /// Cpu状態をリセットする関数
    /// # Returns
    /// * リセットされたCpu構造体
    /// # Example
    /// ```
    /// let reset_cpu = Cpu::reset();
    /// assert_eq!(reset_cpu.pc_counter, 0);
    /// assert_eq!(reset_cpu.a_reg, 0);
    /// assert_eq!(reset_cpu.c_flag, false);
    /// ```
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
            pc_counter: 0,
            // 汎用レジスタAを0に初期化
            a_reg: 0,
            // キャリーフラグをfalseに初期化
            c_flag: false,
        }
    }
}
