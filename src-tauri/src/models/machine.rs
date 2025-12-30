use std::default;

use crate::models::bit::U4;
use crate::models::Cpu;
use crate::models::IoPort;

/// マシン構造体
#[derive(Debug)]
pub struct Machine {
    /// CPU構造体
    pub cpu: Cpu,
    /// IOポート構造体
    pub io_port: IoPort,
    /// ROMデータ（4bit x 4）
    pub rom: [U4; 4],
}

impl Machine {
    /// Machine状態をリセットする関数
    /// # Returns
    /// * リセットされたMachine構造体
    pub fn reset() -> Self {
        Self::default()
    }
}

impl default::Default for Machine {
    /// デフォルトのマシン状態を返す
    /// # Returns
    /// * デフォルト状態のMachine構造体
    fn default() -> Self {
        Machine {
            // デフォルトのCPU状態を初期化
            cpu: Cpu::default(),
            // デフォルトのIOポート状態を初期化
            io_port: IoPort::default(),
            // ROMをゼロで初期化
            rom: [U4::mask(0); 4],
        }
    }
}
