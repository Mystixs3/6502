# 6502

An assembler for the MOS 6502 microprocessor.

I want this to eventually turn out to be something more CPU agnostic. (e.g x86, arm) But thats probably dreaming a bit too big.

The way this would work is it would become a bit more modular and a bit more compiler-like. For example, there would be a module that parses the input assembly, a tokeniser to make some kind of token system from the input, and then the CPU specific modules that turns those tokens into the relevant machine code.

Yeah probably dreaming to big.