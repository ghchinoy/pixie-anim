//! Bit-level utilities for packing variable-length LZW codes.

pub struct BitWriter<'a> {
    buffer: &'a mut Vec<u8>,
    current_byte: u8,
    bit_offset: u8,
}

impl<'a> BitWriter<'a> {
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self {
            buffer,
            current_byte: 0,
            bit_offset: 0,
        }
    }

    /// Writes `bits` value using `n_bits` bit length.
    /// GIF LZW uses LSB-first bit packing.
    pub fn write_bits(&mut self, mut bits: u16, mut n_bits: u8) {
        while n_bits > 0 {
            let space_in_byte = 8 - self.bit_offset;
            let bits_to_write = n_bits.min(space_in_byte);
            
            // Mask the bits we are writing
            let mask = (1 << bits_to_write) - 1;
            let value = (bits & mask) as u8;
            
            self.current_byte |= value << self.bit_offset;
            
            self.bit_offset += bits_to_write;
            bits >>= bits_to_write;
            n_bits -= bits_to_write;

            if self.bit_offset == 8 {
                self.buffer.push(self.current_byte);
                self.current_byte = 0;
                self.bit_offset = 0;
            }
        }
    }

    pub fn flush(&mut self) {
        if self.bit_offset > 0 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_offset = 0;
        }
    }
}
