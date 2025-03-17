mod instructions;
pub struct Cpu {
    memory: [u8; 2048],
    program_counter: u16,
    accumulator: u8,
    stack_pointer: u8,
    status_register: u8,
    x_index: u8,
    y_index: u8,
}

pub enum AddressingMode {
    /// For instructions with no operand.
    ///
    /// Instructions like RTS or CLC have no address operand, the destination of results are implied.
    Implicit,
    /// Operand embedded in the instruction.
    ///
    /// Uses the 8-bit operand itself as the value for the operation, rather than fetching a value from a memory address.
    Immediate,
    /// Addressing within the zero page with optional X or Y offsets.
    ///
    /// Fetches the value from an 8-bit address on the zero page.
    ZeroPage,
    /// Addressing within the zero page with optional X or Y offsets.
    ///
    /// formula: val = PEEK((arg + X) % 256). 4 cpu cycles
    ZeroPageX,
    /// Addressing within the zero page with optional X or Y offsets.
    ///
    /// formula: val = PEEK((arg + Y) % 256). 4 cpu cylces
    ZeroPageY,
    /// Full 16-bit addresses with optional X or Y offsets (and potential extra cycles on page crossing).
    ///
    /// Fetches the value from a 16-bit address anywhere in memory.
    Absolute,
    /// Full 16-bit addresses with optional X or Y offsets (and potential extra cycles on page crossing).
    ///
    /// fomrula: val = PEEK(arg + X). 4+ cpu cycles
    AbsoluteX,
    /// Full 16-bit addresses with optional X or Y offsets (and potential extra cycles on page crossing).
    ///
    /// fomrula: val = PEEK(arg + Y). 4+ cpu cycles
    AbsoluteY,
    /// The special indirect mode used by JMP (noting the 6502 bug when the pointer is at a page boundary).
    ///
    /// The JMP instruction has a special indirect addressing mode that can jump to the address stored in a 16-bit pointer anywhere in memory.
    Indirect,
    /// formula: val = PEEK(PEEK((arg + X) % 256) + PEEK((arg + X + 1) % 256) * 256). 6 cpu cylces
    IndexedIndirect,
    /// formula: = val = PEEK(PEEK(arg) + PEEK((arg + 1) % 256) * 256 + Y). 5+ cpu cylces
    IndirectIndexed,
    /// Used for branch instructions with an 8-bit signed offset.
    ///
    /// Branch instructions (e.g. BEQ, BCS) have a relative addressing mode that specifies an 8-bit signed offset relative to the current PC.
    Relative,
}
