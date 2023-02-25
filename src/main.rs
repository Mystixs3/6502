use rs_6502::*;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[macro_use]
extern crate scan_fmt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path: &String;
    let output_path = Path::new("./a.out");

    if args.len() >= 2 {
        input_path = &args[1];
    } else {
        println!("Not enough arguments given");
        return;
    }

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut output_file = match File::create(&output_path) {
        Err(why) => panic!("couldn't create {}: {}", output_path.display(), why),
        Ok(file) => file,
    };

    let mut output: Instruction;
    let mut input: Mnemonic;
    let mut pc: u16 = 0x0000;

    if let Ok(lines) = read_lines(input_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(line) = line {
                let (tempa, tempb) = scan_fmt_some!(&line, "{} {}", String, String);

                match tempb {
                    Some(x) => {
                        input = Mnemonic {
                            inst: tempa.unwrap(),
                            args: Some(x),
                        }
                    }
                    None => {
                        input = Mnemonic {
                            inst: tempa.unwrap(),
                            args: None,
                        }
                    }
                }

                match assemble(input, pc) {
                    Ok(x) => {
                        output = x;
                        pc = calc_new_pc(pc, &output);
                    }
                    Err(e) => {
                        println!("{}", e);
                        return;
                    }
                }

                output_file
                    .write(&output.opcode.to_le_bytes())
                    .expect("write failed");

                match &output.operand.unwrap() {
                    Operand::Full(x) => {
                        output_file.write(&x.to_le_bytes()).expect("write failed");
                    }
                    Operand::Half(x) => {
                        output_file.write(&x.to_le_bytes()).expect("write failed");
                    }
                }
            }
        }
    }
}

fn calc_new_pc(pc: u16, input: &Instruction) -> u16 {
    match &input.operand {
        Some(x) => match x {
            Operand::Half(..) => pc + 1 + 1,
            Operand::Full(..) => pc + 1 + 2,
        },
        _ => pc + 1,
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
