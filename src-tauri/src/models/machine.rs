use std::default;

use crate::models::Cpu;
use crate::models::IoPort;

/// マシン構造体
pub struct Machine {
    /// CPU構造体
    pub cpu: Cpu,
    /// IOポート構造体
    pub io_port: IoPort,
    /// ROMデータ（4bit x 4）
    pub rom: [u8; 4],
}

impl Machine {
    /// Machine状態をリセットする関数
    /// # Returns
    /// * リセットされたMachine構造体
    /// # Example
    /// ```
    /// let reset_machine = Machine::reset();
    /// assert_eq!(reset_machine.cpu.pc_counter, 0);
    /// assert_eq!(reset_machine.cpu.a_reg, 0);
    /// assert_eq!(reset_machine.cpu.c_flag, false);
    /// assert_eq!(reset_machine.io_port.input_port, 0);
    /// assert_eq!(reset_machine.io_port.output_port, 0);
    /// assert_eq!(reset_machine.rom, [0; 4]);
    /// ```
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
            rom: [0; 4],
        }
    }
}
