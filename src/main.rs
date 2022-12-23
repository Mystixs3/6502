use rs_6502::*;
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();

    let input = Mnemonic {
		inst: args[1].clone().to_uppercase(),
		args: Some(args[2].clone().to_uppercase()),
	};

	let output: Instruction;

	match assemble(input) {
		Ok(x) => output = x,
		Err(e) => { println!("{}", e); return; },
	}

	dbg!(output);
}
