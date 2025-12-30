use crate::models::{Cpu, IoPort, Machine, U2, U4};

/// マシンを1ティック進める関数
/// # Arguments
/// * `machine` - マシン状態
/// # Returns
/// * 命令実行後の更新されたマシン状態
pub fn tick(machine: &Machine) -> Machine {
    // 命令をフェッチ
    let instruction = fetch_instruction(machine);

    // 命令をデコード
    let (opcode, operand) = decode_instruction(instruction);

    // 命令を実行
    match opcode.get() {
        0b00 => {
            // ADD Im
            let new_cpu = execute_add(&machine.cpu, operand);
            Machine {
                cpu: new_cpu,
                io_port: machine.io_port,
                rom: machine.rom,
            }
        }
        0b01 => {
            // IN A
            let new_cpu = execute_in(&machine.cpu, machine.io_port.input_port);
            Machine {
                cpu: new_cpu,
                io_port: machine.io_port,
                rom: machine.rom,
            }
        }
        0b10 => {
            // OUT Im
            let (new_cpu, output_value) = execute_out(&machine.cpu, operand);
            Machine {
                cpu: new_cpu,
                io_port: IoPort {
                    input_port: machine.io_port.input_port,
                    output_port: output_value,
                },
                rom: machine.rom,
            }
        }
        0b11 => {
            // JNC Addr
            let new_cpu = execute_jnc(&machine.cpu, operand);
            Machine {
                cpu: new_cpu,
                io_port: machine.io_port,
                rom: machine.rom,
            }
        }
        _ => Machine {
            cpu: Cpu {
                pc_counter: machine.cpu.pc_counter,
                a_reg: machine.cpu.a_reg,
                c_flag: machine.cpu.c_flag,
            },
            io_port: machine.io_port,
            rom: machine.rom,
        },
    }
}

/// 命令をデコードする関数
/// # Arguments
/// * `instruction` - 4ビット命令
/// # Returns
/// * (オペコード, オペランド)のタプル
fn decode_instruction(instruction: U4) -> (U2, U2) {
    let opcode = instruction.get() >> 2;
    let operand = instruction.get();
    (U2::mask(opcode), U2::mask(operand))
}

/// ROMから命令をフェッチする関数
/// # Arguments
/// * `machine` - マシン状態
/// # Returns
/// * フェッチされた命令
fn fetch_instruction(machine: &Machine) -> U4 {
    machine.rom[machine.cpu.pc_counter.get() as usize]
}

/// ADD命令を実行する関数
/// {C, A} ← A + Im
fn execute_add(cpu: &Cpu, operand: U2) -> Cpu {
    let sum = (cpu.a_reg.get() as u16) + (operand.get() as u16);
    let c_flag = sum > 0b11;

    Cpu {
        pc_counter: U2::mask(cpu.pc_counter.get() + 1),
        a_reg: U2::mask(sum as u8),
        c_flag,
    }
}

/// IN命令を実行する関数
/// A ← INPUT
/// C ← 0
fn execute_in(cpu: &Cpu, input_port: U2) -> Cpu {
    Cpu {
        pc_counter: U2::mask(cpu.pc_counter.get() + 1),
        a_reg: input_port,
        c_flag: false,
    }
}

/// OUT命令を実行する関数
/// OUTPUT ← Im
/// C ← 0
fn execute_out(cpu: &Cpu, output_value: U2) -> (Cpu, U2) {
    let new_cpu = Cpu {
        pc_counter: U2::mask(cpu.pc_counter.get() + 1),
        a_reg: cpu.a_reg,
        c_flag: false,
    };
    (new_cpu, output_value)
}

/// JNC命令を実行する関数
/// if C == 0:
///     PC ← Addr
/// else:
///     PC ← PC + 1
/// C ← 0
fn execute_jnc(cpu: &Cpu, addr: U2) -> Cpu {
    let new_pc = if !cpu.c_flag {
        addr // キャリーフラグが0ならアドレスにジャンプ
    } else {
        U2::mask(cpu.pc_counter.get() + 1) // キャリーフラグが1なら次の命令へ
    };

    Cpu {
        pc_counter: new_pc,
        a_reg: cpu.a_reg,
        c_flag: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// decode_instruction が正しくデコードすることを確認するテスト
    #[test]
    fn test_decode_instruction() {
        assert_eq!(
            decode_instruction(U4::mask(0b00_00)),
            (U2::mask(0b00), U2::mask(0b00))
        );
        assert_eq!(
            decode_instruction(U4::mask(0b00_11)),
            (U2::mask(0b00), U2::mask(0b11))
        );
        assert_eq!(
            decode_instruction(U4::mask(0b01_10)),
            (U2::mask(0b01), U2::mask(0b10))
        );
        assert_eq!(
            decode_instruction(U4::mask(0b11_01)),
            (U2::mask(0b11), U2::mask(0b01))
        );
    }

    /// fetch_instruction がROMから正しく命令を取得することを確認するテスト
    #[test]
    fn test_fetch_instruction() {
        let mut machine = Machine::default();
        machine.rom[0] = U4::mask(0b00_01);
        machine.rom[1] = U4::mask(0b01_10);
        machine.rom[2] = U4::mask(0b10_11);
        machine.rom[3] = U4::mask(0b11_00);

        machine.cpu.pc_counter = U2::mask(0);
        assert_eq!(fetch_instruction(&machine), U4::mask(0b00_01));

        machine.cpu.pc_counter = U2::mask(1);
        assert_eq!(fetch_instruction(&machine), U4::mask(0b01_10));

        machine.cpu.pc_counter = U2::mask(2);
        assert_eq!(fetch_instruction(&machine), U4::mask(0b10_11));

        machine.cpu.pc_counter = U2::mask(3);
        assert_eq!(fetch_instruction(&machine), U4::mask(0b11_00));
    }

    /// ADD命令の実行をテスト
    #[test]
    fn test_execute_add() {
        let cpu = Cpu {
            pc_counter: U2::mask(0),
            a_reg: U2::mask(0b01),
            c_flag: false,
        };

        let result = execute_add(&cpu, U2::mask(0b10));
        assert_eq!(result.a_reg, U2::mask(0b11)); // 1 + 2 = 3
        assert_eq!(result.c_flag, false); // キャリーなし
        assert_eq!(result.pc_counter, U2::mask(1)); // PC + 1
    }

    /// ADD命令でキャリーが発生することを確認するテスト
    #[test]
    fn test_execute_add_with_carry() {
        let cpu = Cpu {
            pc_counter: U2::mask(1),
            a_reg: U2::mask(0b11),
            c_flag: false,
        };

        let result = execute_add(&cpu, U2::mask(0b01));
        assert_eq!(result.a_reg, U2::mask(0b00)); // 3 + 1 = 4, マスクされて 0
        assert_eq!(result.c_flag, true); // キャリーあり
        assert_eq!(result.pc_counter, U2::mask(2)); // PC + 1
    }

    /// IN命令の実行をテスト
    #[test]
    fn test_execute_in() {
        let cpu = Cpu {
            pc_counter: U2::mask(2),
            a_reg: U2::mask(0b11),
            c_flag: true,
        };

        let result = execute_in(&cpu, U2::mask(0b10));
        assert_eq!(result.a_reg, U2::mask(0b10)); // 入力ポート値
        assert_eq!(result.c_flag, false); // キャリーリセット
        assert_eq!(result.pc_counter, U2::mask(3)); // PC + 1
    }

    /// OUT命令の実行をテスト
    #[test]
    fn test_execute_out() {
        let cpu = Cpu {
            pc_counter: U2::mask(3),
            a_reg: U2::mask(0b01),
            c_flag: true,
        };

        let (result_cpu, output_value) = execute_out(&cpu, U2::mask(0b11));
        assert_eq!(result_cpu.a_reg, U2::mask(0b01)); // A レジスタは変わらない
        assert_eq!(result_cpu.c_flag, false); // キャリーリセット
        assert_eq!(result_cpu.pc_counter, U2::mask(0)); // PC + 1 = 4, マスクされて 0
        assert_eq!(output_value, U2::mask(0b11)); // 出力値
    }

    /// JNC命令でキャリーフラグが0の場合（ジャンプ）のテスト
    #[test]
    fn test_execute_jnc_jump() {
        let cpu = Cpu {
            pc_counter: U2::mask(1),
            a_reg: U2::mask(0b10),
            c_flag: false, // キャリーなし -> ジャンプ
        };

        let result = execute_jnc(&cpu, U2::mask(0b11));
        assert_eq!(result.pc_counter, U2::mask(0b11)); // アドレスにジャンプ
        assert_eq!(result.c_flag, false); // キャリーリセット
        assert_eq!(result.a_reg, U2::mask(0b10)); // A レジスタは変わらない
    }

    /// JNC命令でキャリーフラグが1の場合（次の命令へ）のテスト
    #[test]
    fn test_execute_jnc_no_jump() {
        let cpu = Cpu {
            pc_counter: U2::mask(1),
            a_reg: U2::mask(0b10),
            c_flag: true, // キャリーあり -> ジャンプしない
        };

        let result = execute_jnc(&cpu, U2::mask(0b11));
        assert_eq!(result.pc_counter, U2::mask(2)); // PC + 1
        assert_eq!(result.c_flag, false); // キャリーリセット
        assert_eq!(result.a_reg, U2::mask(0b10)); // A レジスタは変わらない
    }

    /// tick関数でADD命令を実行するテスト
    #[test]
    fn test_tick_add() {
        let mut machine = Machine::default();
        machine.rom[0] = U4::mask(0b00_01); // ADD 1
        machine.cpu.a_reg = U2::mask(0b10); // Aレジスタに2をセット

        let new_machine = tick(&machine);
        assert_eq!(new_machine.cpu.a_reg, U2::mask(0b11)); // 2 + 1 = 3
        assert_eq!(new_machine.cpu.c_flag, false);
        assert_eq!(new_machine.cpu.pc_counter, U2::mask(1));
    }

    /// tick関数でIN命令を実行するテスト
    #[test]
    fn test_tick_in() {
        let mut machine = Machine::default();
        machine.rom[0] = U4::mask(0b01_00); // IN
        machine.io_port.input_port = U2::mask(0b10); // 入力ポートに2をセット

        let new_machine = tick(&machine);
        assert_eq!(new_machine.cpu.a_reg, U2::mask(0b10)); // 入力ポート値
        assert_eq!(new_machine.cpu.c_flag, false);
        assert_eq!(new_machine.cpu.pc_counter, U2::mask(1));
    }

    /// tick関数でOUT命令を実行するテスト
    #[test]
    fn test_tick_out() {
        let mut machine = Machine::default();
        machine.rom[0] = U4::mask(0b10_11); // OUT 3

        let new_machine = tick(&machine);
        assert_eq!(new_machine.io_port.output_port, U2::mask(0b11)); // 出力ポート値
        assert_eq!(new_machine.cpu.c_flag, false);
        assert_eq!(new_machine.cpu.pc_counter, U2::mask(1));
    }

    /// tick関数でJNC命令を実行するテスト
    #[test]
    fn test_tick_jnc() {
        let mut machine = Machine::default();
        machine.rom[0] = U4::mask(0b11_10); // JNC 2
        machine.cpu.c_flag = false; // ジャンプする

        let new_machine = tick(&machine);
        assert_eq!(new_machine.cpu.pc_counter, U2::mask(0b10)); // ジャンプ先アドレス
        assert_eq!(new_machine.cpu.c_flag, false);
    }
}
