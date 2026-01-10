//! Bit-level reading and writing utilities.

/// A writer that packs variable-length bits into a byte stream.
pub struct BitWriter<'a> {
    buffer: &'a mut Vec<u8>,
    current_byte: u8,
    bit_count: u8,
}

impl<'a> BitWriter<'a> {
    /// Creates a new BitWriter wrapping the provided byte buffer.
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self {
            buffer,
            current_byte: 0,
            bit_count: 0,
        }
    }

    /// Writes `count` bits from `value` to the stream.
    pub fn write_bits(&mut self, value: u16, count: u8) {
        let mut remaining_bits = count;
        let mut val = value as u32;

        while remaining_bits > 0 {
            let bits_to_write = std::cmp::min(remaining_bits, 8 - self.bit_count);
            let mask = (1 << bits_to_write) - 1;

            self.current_byte |= ((val & mask) as u8) << self.bit_count;

            self.bit_count += bits_to_write;
            val >>= bits_to_write;
            remaining_bits -= bits_to_write;

            if self.bit_count == 8 {
                self.buffer.push(self.current_byte);
                self.current_byte = 0;
                self.bit_count = 0;
            }
        }
    }

    /// Flushes any remaining bits to the buffer, padding with zeros if necessary.
    pub fn flush(&mut self) {
        if self.bit_count > 0 {
            self.buffer.push(self.current_byte);
            self.current_byte = 0;
            self.bit_count = 0;
        }
    }
}
