//! GIF89a Structure and Writing.

use crate::error::Result;
use crate::lzw::LzwEncoder;

pub struct GifOptions {
    pub width: u16,
    pub height: u16,
    pub has_global_palette: bool,
    pub palette_size: u8, // power of 2: 2^(n+1)
}

pub struct GifWriter<'a> {
    buffer: &'a mut Vec<u8>,
}

impl<'a> GifWriter<'a> {
    pub fn new(buffer: &'a mut Vec<u8>) -> Self {
        Self { buffer }
    }

    pub fn write_header(&mut self) -> Result<()> {
        self.buffer.extend_from_slice(b"GIF89a");
        Ok(())
    }

    pub fn write_logical_screen_descriptor(&mut self, options: &GifOptions) -> Result<()> {
        self.buffer.extend_from_slice(&options.width.to_le_bytes());
        self.buffer.extend_from_slice(&options.height.to_le_bytes());
        
        let mut packed = 0u8;
        if options.has_global_palette {
            packed |= 0x80;
            packed |= (options.palette_size - 1) & 0x07;
            packed |= 0x70; // 8-bit color resolution
        }
        
        self.buffer.push(packed);
        self.buffer.push(0); // background color index
        self.buffer.push(0); // pixel aspect ratio
        Ok(())
    }

    pub fn write_netscape_loop_block(&mut self) -> Result<()> {
        self.buffer.push(0x21); // extension introducer
        self.buffer.push(0xFF); // application extension label
        self.buffer.push(0x0B); // block size (11 bytes)
        self.buffer.extend_from_slice(b"NETSCAPE2.0");
        self.buffer.push(0x03); // sub-block size
        self.buffer.push(0x01); // loop sub-block id
        self.buffer.extend_from_slice(&0u16.to_le_bytes()); // loop count (0 = infinite)
        self.buffer.push(0); // block terminator
        Ok(())
    }

    pub fn write_global_palette(&mut self, palette: &[u8]) -> Result<()> {
        self.buffer.extend_from_slice(palette);
        Ok(())
    }

    pub fn write_graphic_control_extension(&mut self, delay: u16, transparent_idx: Option<u8>) -> Result<()> {
        self.buffer.push(0x21); // extension introducer
        self.buffer.push(0xF9); // graphic control label
        self.buffer.push(0x04); // block size
        
        let mut packed = 0x04; // Disposal Method: 1 (Do not dispose)
        if transparent_idx.is_some() {
            packed |= 0x01;
        }
        self.buffer.push(packed);
        self.buffer.extend_from_slice(&delay.to_le_bytes());
        self.buffer.push(transparent_idx.unwrap_or(0));
        self.buffer.push(0); // block terminator
        Ok(())
    }

    pub fn write_image_data(&mut self, x: u16, y: u16, width: u16, height: u16, lzw_min_code_size: u8, indices: &[u8], encoder: &mut LzwEncoder) -> Result<()> {
        // Image Descriptor
        self.buffer.push(0x2C); // separator
        self.buffer.extend_from_slice(&x.to_le_bytes());
        self.buffer.extend_from_slice(&y.to_le_bytes());
        self.buffer.extend_from_slice(&width.to_le_bytes());
        self.buffer.extend_from_slice(&height.to_le_bytes());
        self.buffer.push(0); // packed: no local palette

        // LZW Minimum Code Size
        self.buffer.push(lzw_min_code_size);

        // Encode data into sub-blocks
        let mut lzw_data = Vec::new();
        encoder.encode(indices, &mut lzw_data)?;

        // GIF requires data in sub-blocks (max 255 bytes)
        for chunk in lzw_data.chunks(255) {
            self.buffer.push(chunk.len() as u8);
            self.buffer.extend_from_slice(chunk);
        }
        self.buffer.push(0); // block terminator

        Ok(())
    }

    pub fn write_trailer(&mut self) -> Result<()> {
        self.buffer.push(0x3B);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_minimal_gif() {
        let mut buffer = Vec::new();
        let mut writer = GifWriter::new(&mut buffer);
        let options = GifOptions {
            width: 1,
            height: 1,
            has_global_palette: true,
            palette_size: 1, // 2 colors
        };
        
        writer.write_header().unwrap();
        writer.write_logical_screen_descriptor(&options).unwrap();
        
        let mut palette = vec![0u8; 6]; // 2 RGB colors
        palette[0] = 255; // Red
        writer.write_global_palette(&palette).unwrap();
        
        let mut encoder = LzwEncoder::new(2);
        writer.write_image_data(0, 0, 1, 1, 2, &[0], &mut encoder).unwrap();
        writer.write_trailer().unwrap();
        
        assert!(buffer.starts_with(b"GIF89a"));
        assert_eq!(*buffer.last().unwrap(), 0x3B);
    }
}
