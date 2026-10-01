use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let data = std::fs::read(file_path).unwrap();
    let mut result: Vec<String> = Vec::new();

    for i in (0..data.len() - 1).step_by(2) {
        result.push(disassemble([data[i], data[i + 1]]));
    }

    println!("bits 16\n");
    for line in result {
        println!("{}", line);
    }
}

fn disassemble(data: [u8; 2]) -> String {
    if data[0] & 0b11111100 == 0b10001000 {
        return disassemble_mov(data);
    }
    todo!()
}

fn disassemble_mov(data: [u8; 2]) -> String {
    let mut result = "mov ".to_string();
    let d = (data[0] & 0b10) >> 1;
    let w = data[0] & 0b1;
    let _mod = data[1] >> 6;
    let reg = (data[1] & 0b00111000) >> 3;
    let rm = data[1] & (0b111);

    let reg_str = if w == 0 {
        match reg {
            0b000 => "al",
            0b001 => "cl",
            0b010 => "dl",
            0b011 => "bl",
            0b100 => "ah",
            0b101 => "ch",
            0b110 => "dh",
            0b111 => "bh",
            _ => unreachable!(),
        }
    } else {
        match reg {
            0b000 => "ax",
            0b001 => "cx",
            0b010 => "dx",
            0b011 => "bx",
            0b100 => "sp",
            0b101 => "bp",
            0b110 => "si",
            0b111 => "di",
            _ => unreachable!(),
        }
    };

    let rm_str = if w == 0 {
        match rm {
            0b000 => "al",
            0b001 => "cl",
            0b010 => "dl",
            0b011 => "bl",
            0b100 => "ah",
            0b101 => "ch",
            0b110 => "dh",
            0b111 => "bh",
            _ => unreachable!(),
        }
    } else {
        match rm {
            0b000 => "ax",
            0b001 => "cx",
            0b010 => "dx",
            0b011 => "bx",
            0b100 => "sp",
            0b101 => "bp",
            0b110 => "si",
            0b111 => "di",
            _ => unreachable!(),
        }
    };

    if d == 1 {
        result.push_str(&format!("{}, {}", reg_str, rm_str));
    } else {
        result.push_str(&format!("{}, {}", rm_str, reg_str));
    };

    result
}
