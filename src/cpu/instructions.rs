//! See: https://www.nesdev.org/wiki/Instruction_reference
use crate::cpu::Cpu;
impl Cpu {
    /// ADC - Add with Carry
    ///
    /// `A = A + memory + C`
    ///
    /// ADC adds the carry flag and a memory value to the accumulator.
    ///
    /// - The carry flag is then set to the carry value coming out of bit 7
    ///   - this allows values larger than 1 byte to be added together by carrying the 1 into the next byte's addition.
    /// - This can also be thought of as unsigned overflow.
    /// - It is common to clear carry with CLC before adding the first byte to ensure it is in a known state, avoiding an off-by-one error.
    /// - The overflow flag indicates whether signed overflow or underflow occurred.
    /// - This happens if both inputs are positive and the result is negative, or both are negative and the result is positive.
    ///
    /// ### Status Flags
    ///
    /// | Flag | New Value | Notes |
    /// |------|----------|-------------------------------------------------------------|
    /// | C - Carry | result > $FF | If the result overflowed past $FF (wrapping around), unsigned overflow occurred. |
    /// | Z - Zero | result == 0 |  |
    /// | V - Overflow | (result ^ A) & (result ^ memory) & $80 | If the result's sign is different from both A's and memory's, signed overflow (or underflow) occurred. |
    /// | N - Negative | result bit 7 |  |
    ///
    /// ### Addressing Modes
    ///
    /// | Addressing Mode | Opcode | Bytes | Cycles |
    /// |----------------|--------|-------|--------|
    /// | #Immediate | $69 | 2 | 2 |
    /// | Zero Page | $65 | 2 | 3 |
    /// | Zero Page,X | $75 | 2 | 4 |
    /// | Absolute | $6D | 3 | 4 |
    /// | Absolute,X | $7D | 3 | 4 (5 if page crossed) |
    /// | Absolute,Y | $79 | 3 | 4 (5 if page crossed) |
    /// | (Indirect,X) | $61 | 2 | 6 |
    /// | (Indirect),Y | $71 | 2 | 5 (6 if page crossed) |
    pub fn adc(&mut self) {

    }
    pub fn and(&self) {}
    pub fn asl(&self) {}
    pub fn bcc(&self) {}
    pub fn bcs(&self) {}
    pub fn beq(&self) {}
    pub fn bit(&self) {}
    pub fn bmi(&self) {}
    pub fn bne(&self) {}
    pub fn bpl(&self) {}
    pub fn brk(&self) {}
    pub fn bvc(&self) {}
    pub fn bvs(&self) {}
    pub fn clc(&self) {}
    pub fn cld(&self) {}
    pub fn cli(&self) {}
    pub fn clv(&self) {}
    pub fn cmp(&self) {}
    pub fn cpx(&self) {}
    pub fn cpy(&self) {}
    pub fn dec(&self) {}
    pub fn dex(&self) {}
    pub fn dey(&self) {}
    pub fn eor(&self) {}
    pub fn inc(&self) {}
    pub fn inx(&self) {}
    pub fn iny(&self) {}
    pub fn jmp(&self) {}
    pub fn jsr(&self) {}
    pub fn lda(&self) {}
    pub fn ldx(&self) {}
    pub fn ldy(&self) {}
    pub fn lsr(&self) {}
    pub fn nop(&self) {}
    pub fn ora(&self) {}
    pub fn pha(&self) {}
    pub fn php(&self) {}
    pub fn pla(&self) {}
    pub fn plp(&self) {}
    pub fn rol(&self) {}
    pub fn ror(&self) {}
    pub fn rti(&self) {}
    pub fn rts(&self) {}
    pub fn sbc(&self) {}
    pub fn sec(&self) {}
    pub fn sed(&self) {}
    pub fn sei(&self) {}
    pub fn sta(&self) {}
    pub fn stx(&self) {}
    pub fn sty(&self) {}
    pub fn tax(&self) {}
    pub fn tay(&self) {}
    pub fn tsx(&self) {}
    pub fn txa(&self) {}
    pub fn txs(&self) {}
    pub fn tya(&self) {}
}
