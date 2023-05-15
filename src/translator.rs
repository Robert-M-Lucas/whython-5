macro_rules! translate {
    ($instruction: ident, $data: expr, $i: expr) => {
        (
            $instruction::get_debug(&$data[$i..$i + $instruction::get_size()]),
            $instruction::get_size(),
        )
    };
}

/// Prints the instructions and their data in the given memory
pub fn translate(data: &[u8], translate_one: bool) {
    println!("<------------------------------>");
    let mut i: usize = 0;
    while i < data.len() {
        print!("[{:0>5}] | ", i);

        let code = &data[i..i + 2];
        i += 2;
        let (output, size): (String, usize) = match u16::from_le_bytes(code.try_into().unwrap()) {
            code => panic!("Debug not implemented for code {}", code),
        };

        println!("{}", output);

        if translate_one {
            break;
        }

        i += size;
    }
    println!("<------------------------------>");
}
