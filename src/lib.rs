#[macro_use] extern crate scan_fmt;

#[derive(Debug)]
pub struct Mnemonic {
	pub inst: String,
	pub args: Option<String>,
}

#[derive(Debug)]
pub struct Instruction {
	pub opcode: u8,
	pub operand: Option<Operand>,
}

#[derive(Debug)]
pub enum Operand {
	Half(u8),
	Full(u16),
}

pub fn assemble(input: Mnemonic) -> Result<Instruction, String> {
	let mut output = Instruction { opcode: 0x00, operand: None };

	dbg!(&input);

	// BRK [impl]
	if input.inst == "BRK" {}
	// ORA
	else if input.inst == "ORA" {
		// ORA (ind,X)
		if input.args.clone().unwrap_or(String::from("")).starts_with("($") && input.args.clone().unwrap_or(String::from("")).ends_with(",X)") {
			output.opcode = 0x01;
			output.operand = Some(Operand::Half(scan_fmt!(&input.args.clone().unwrap(), "(${},X)", [hex u8]).unwrap()));
		}
		// ORA zpg
		if input.args.clone().unwrap_or(String::from("")).starts_with("$") {
			output.opcode = 0x05;
			output.operand = Some(Operand::Half(scan_fmt!(&input.args.clone().unwrap(), "${}", [hex u8]).unwrap()));
		}
	}

	else {
		return Err(String::from("The inputted mnemonic didn't match with anything"));
	}
	
	Ok(output)
}