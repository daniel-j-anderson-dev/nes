mod instructions;
pub struct Cpu {
    accumulator: u8,
    x_index: u8,
    y_index: u8,
    program_counter: u16,
    stack_pointer: u8,
    status_register: u8,
}

