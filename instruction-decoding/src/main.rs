fn main() {
    let text = std::fs::read_to_string("./public/listing_0038_many_register_mov.asm").unwrap();
    let lines = text.lines().collect::<Vec<_>>();

    for line in lines {
        let bytes = decode_operation(line);

        println!("{:b} {:b}", bytes[0], bytes[1]);
    }
}

fn decode_operation(op: &str) -> [u8; 2] {
    let mut value = [0, 0];

    value[0] = 0b100010 << 2;
    value[0] |= 0b00000010;
    let op = &op[3..];

    let (reg, rm) = {
        let split = op.split(',').collect::<Vec<_>>();
        (split[0].trim(), split[1].trim())
    };

    // println!("reg string: {}", reg);
    // println!("rm string: {}", rm);

    let (reg, w1) = encode_reg(reg);
    let (rm, w2) = encode_reg(rm);
    let w = w1 || w2;
    // println!("reg: {:08b}", reg);
    // println!("rm: {:08b}", rm);

    value[0] |= w as u8;

    // Here goes the MOD
    value[1] = 0b11 << 6;

    // REG and RM
    value[1] |= reg << 3;
    value[1] |= rm;

    value
}

fn encode_reg(reg: &str) -> (u8, bool) {
    match reg {
        "al" => (0b0, false),
        "cl" => (0b1, false),
        "dl" => (0b10, false),
        "bl" => (0b11, false),
        "ah" => (0b100, false),
        "ch" => (0b101, false),
        "dh" => (0b110, false),
        "bh" => (0b111, false),
        "ax" => (0b0, true),
        "cx" => (0b1, true),
        "dx" => (0b10, true),
        "bx" => (0b11, true),
        "sp" => (0b100, true),
        "bp" => (0b101, true),
        "si" => (0b110, true),
        "di" => (0b111, true),
        _ => (0b0, false),
    }
}
