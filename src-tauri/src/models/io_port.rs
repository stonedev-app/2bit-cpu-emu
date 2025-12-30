use crate::models::U2;

/// IOポート構造体
#[derive(Debug, Clone, Copy)]
pub struct IoPort {
    /// 入力ポート(2bit)
    pub input_port: U2,
    /// 出力ポート(2bit)
    pub output_port: U2,
}

impl IoPort {
    /// IoPort状態をリセットする関数
    /// # Returns
    /// * リセットされたIoPort構造体
    pub fn reset() -> Self {
        Self::default()
    }
}

impl Default for IoPort {
    /// デフォルトのIOポート状態を返す
    /// # Returns
    /// * デフォルト状態のIoPort構造体
    fn default() -> Self {
        IoPort {
            // 入力ポートを0に初期化
            input_port: U2::mask(0),
            // 出力ポートを0に初期化
            output_port: U2::mask(0),
        }
    }
}
