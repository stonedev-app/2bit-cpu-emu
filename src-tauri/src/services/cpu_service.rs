use crate::models::Cpu;
use crate::services::pc_service::inc_pc;

pub fn tick(cpu: &Cpu) -> Cpu {
    // PCインクリメント
    let pc = inc_pc(cpu.pc_counter);
    // 新しいCpuインスタンスを返す
    Cpu { pc_counter: pc }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tick_from_zero() {
        let cpu = Cpu { pc_counter: 0 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 1);
    }

    #[test]
    fn test_tick_from_one() {
        let cpu = Cpu { pc_counter: 1 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 2);
    }

    #[test]
    fn test_tick_from_two() {
        let cpu = Cpu { pc_counter: 2 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 3);
    }

    #[test]
    fn test_tick_from_three_wraps() {
        let cpu = Cpu { pc_counter: 3 };
        let new_cpu = tick(&cpu);
        assert_eq!(new_cpu.pc_counter, 0);
    }

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

    #[test]
    fn test_tick_does_not_modify_original() {
        // tick は元の CPU 構造体を変更しない（イミュータブル）
        let cpu = Cpu { pc_counter: 1 };
        let _new_cpu = tick(&cpu);
        assert_eq!(cpu.pc_counter, 1);
    }
}
