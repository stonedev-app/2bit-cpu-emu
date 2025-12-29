use crate::models::Cpu;
use crate::services::pc_service::inc_pc;

/// CPUのクロックを1ティック進める関数
/// # Arguments
/// * `cpu` - 現在のCPU状態を表すCpu構造体への参照
/// # Returns
/// * 更新されたCPU状態を表す新しいCpu構造体
pub fn tick(cpu: &Cpu) -> Cpu {
    // PCインクリメント
    let pc = inc_pc(cpu.pc_counter);
    // 新しいCpuインスタンスを返す
    Cpu { pc_counter: pc }
}

/// テストモジュール
#[cfg(test)]
mod tests {
    use super::*;

    /// PCカウンターが0から1にインクリメントされることを確認するテスト
    #[test]
    fn test_tick_from_zero() {
        let cpu = Cpu { pc_counter: 0 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 1);
    }

    /// PCカウンターが1から2にインクリメントされることを確認するテスト
    #[test]
    fn test_tick_from_one() {
        let cpu = Cpu { pc_counter: 1 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 2);
    }

    /// PCカウンターが2から3にインクリメントされることを確認するテスト
    #[test]
    fn test_tick_from_two() {
        let cpu = Cpu { pc_counter: 2 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 3);
    }

    /// PCカウンターが3から0にラップアラウンドすることを確認するテスト
    #[test]
    fn test_tick_from_three_wraps() {
        let cpu = Cpu { pc_counter: 3 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 0);
    }

    /// 複数回のティック操作でPCカウンターが正しく更新されることを確認するテスト
    #[test]
    fn test_tick_sequence() {
        // CPUが複数回のティック操作を順次実行
        let mut cpu = Cpu { pc_counter: 0 };

        cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 1);

        cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 2);

        cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 3);

        cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 0);

        cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 1);
    }

    /// 1サイクル（4ティック）が完全に回って元に戻ることを確認するテスト
    #[test]
    fn test_tick_full_cycle() {
        // 1サイクル（4ティック）が完全に回って元に戻る
        let mut cpu = Cpu { pc_counter: 0 };
        let initial_pc = cpu.pc_counter;

        for _ in 0..4 {
            cpu = tick(&cpu);
        }

        assert_eq!(cpu.pc_counter, initial_pc);
    }

    /// tick は元の CPU 構造体を変更しないことを確認するテスト
    #[test]
    fn test_tick_does_not_modify_original() {
        // tick は元の CPU 構造体を変更しない（イミュータブル）
        let cpu = Cpu { pc_counter: 1 };
        let _new_cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 1);
    }
}
