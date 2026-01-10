//! LZW Encoder for GIF89a.
//!
//! Implements variable-length LZW compression with Clear and EOI codes.

use crate::bits::BitWriter;
use crate::error::Result;

const MAX_CODES: u16 = 4096;

/// A Lempel-Ziv-Welch encoder specialized for GIF89a.
pub struct LzwEncoder {
    min_code_size: u8,
    dictionary: Vec<i32>,
    /// Degree of lossiness (0 = lossless, >0 = allow neighbor matching).
    pub lossiness: u8,
}

impl LzwEncoder {
    /// Creates a new LzwEncoder with the specified minimum code size.
    pub fn new(min_code_size: u8) -> Self {
        Self {
            min_code_size,
            dictionary: vec![-1i32; MAX_CODES as usize * 256],
            lossiness: 0,
        }
    }

    /// Compresses a stream of palette indices into the provided buffer.
    pub fn encode(&mut self, data: &[u8], buffer: &mut Vec<u8>) -> Result<()> {
        let clear_code = 1 << self.min_code_size;
        let eoi_code = clear_code + 1;

        let mut bit_writer = BitWriter::new(buffer);
        let mut code_size = self.min_code_size + 1;
        let mut next_code = eoi_code + 1;

        self.dictionary.fill(-1);

        bit_writer.write_bits(clear_code, code_size);

        if data.is_empty() {
            bit_writer.write_bits(eoi_code, code_size);
            bit_writer.flush();
            return Ok(());
        }

        let mut prefix = data[0] as u16;

        for &byte in &data[1..] {
            let character = byte;
            let dict_idx = ((prefix as usize) << 8) | character as usize;
            let code = self.dictionary[dict_idx];

            if code != -1 {
                prefix = code as u16;
            } else {
                // LOSSY OPTIMIZATION:
                // Because we use Zeng Reordering, neighbors (idx-1, idx+1) are visually similar.
                // If we can't find a match for the current byte, check if a neighbor allows
                // us to continue the string.
                let mut found_lossy = false;
                if self.lossiness > 0 {
                    // Try neighbors based on lossiness level
                    for offset in 1..=(self.lossiness as i16) {
                        for sign in &[-1, 1] {
                            let neighbor = character as i16 + (offset * sign);
                            if (0..=255).contains(&neighbor) {
                                let n_idx = ((prefix as usize) << 8) | neighbor as usize;
                                let n_code = self.dictionary[n_idx];
                                if n_code != -1 {
                                    prefix = n_code as u16;
                                    found_lossy = true;
                                    break;
                                }
                            }
                        }
                        if found_lossy {
                            break;
                        }
                    }
                }

                if !found_lossy {
                    bit_writer.write_bits(prefix, code_size);

                    if next_code < MAX_CODES {
                        self.dictionary[dict_idx] = next_code as i32;
                        next_code += 1;

                        if next_code > (1 << code_size) && code_size < 12 {
                            code_size += 1;
                        }
                    } else {
                        bit_writer.write_bits(clear_code, code_size);
                        self.dictionary.fill(-1);
                        code_size = self.min_code_size + 1;
                        next_code = eoi_code + 1;
                    }

                    prefix = character as u16;
                }
            }
        }

        bit_writer.write_bits(prefix, code_size);
        bit_writer.write_bits(eoi_code, code_size);
        bit_writer.flush();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_lzw() {
        let mut encoder = LzwEncoder::new(8);
        let data = vec![1, 1, 1, 1, 1];
        let mut out = Vec::new();
        encoder.encode(&data, &mut out).unwrap();
        assert!(!out.is_empty());
    }
}
