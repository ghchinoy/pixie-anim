//! GIF89a Structure and Writing.

use crate::error::Result;
use crate::lzw::LzwEncoder;
use std::io::Write;

/// Options for configuring the GIF output.
pub struct GifOptions {
    /// Width of the logical screen.
    pub width: u16,
    /// Height of the logical screen.
    pub height: u16,
    /// Whether to include a global color table.
    pub has_global_palette: bool,
    /// Size of the palette as a power of 2 (2^(n+1)).
    pub palette_size: u8,
}

/// Descriptor for a single image frame within the GIF.
pub struct ImageDescriptor {
    /// X offset from the left edge of the logical screen.
    pub x: u16,
    /// Y offset from the top edge of the logical screen.
    pub y: u16,
    /// Width of the image.
    pub width: u16,
    /// Height of the image.
    pub height: u16,
    /// LZW minimum code size.
    pub lzw_min_code_size: u8,
}

/// A writer for creating GIF89a formatted data.
pub struct GifWriter<W: Write> {
    writer: W,
}

impl<W: Write> GifWriter<W> {
    /// Creates a new GifWriter wrapping the provided output stream.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Writes the GIF89a header.
    pub fn write_header(&mut self) -> Result<()> {
        self.writer.write_all(b"GIF89a")?;
        Ok(())
    }

    /// Writes the Logical Screen Descriptor block.
    pub fn write_logical_screen_descriptor(&mut self, options: &GifOptions) -> Result<()> {
        self.writer.write_all(&options.width.to_le_bytes())?;
        self.writer.write_all(&options.height.to_le_bytes())?;

        let mut packed = 0u8;
        if options.has_global_palette {
            packed |= 0x80;
            packed |= (options.palette_size - 1) & 0x07;
            packed |= 0x70; // 8-bit color resolution
        }

        self.writer.write_all(&[packed, 0, 0])?; // packed, background, pixel aspect ratio
        Ok(())
    }

    /// Writes the Netscape Application Block for infinite looping.
    pub fn write_netscape_loop_block(&mut self) -> Result<()> {
        self.writer.write_all(&[0x21, 0xFF, 0x0B])?; // extension introducer, application label, block size
        self.writer.write_all(b"NETSCAPE2.0")?;
        self.writer.write_all(&[0x03, 0x01])?; // sub-block size, loop sub-block id
        self.writer.write_all(&0u16.to_le_bytes())?; // loop count (0 = infinite)
        self.writer.write_all(&[0])?; // block terminator
        Ok(())
    }

    /// Writes the global palette data.
    pub fn write_global_palette(&mut self, palette: &[u8]) -> Result<()> {
        self.writer.write_all(palette)?;
        Ok(())
    }

    /// Writes a Graphic Control Extension block for a frame.
    pub fn write_graphic_control_extension(
        &mut self,
        delay: u16,
        transparent_idx: Option<u8>,
    ) -> Result<()> {
        self.writer.write_all(&[0x21, 0xF9, 0x04])?; // introducer, label, size

        let mut packed = 0x04; // Disposal Method: 1 (Do not dispose)
        if transparent_idx.is_some() {
            packed |= 0x01;
        }
        self.writer.write_all(&[packed])?;
        self.writer.write_all(&delay.to_le_bytes())?;
        self.writer.write_all(&[transparent_idx.unwrap_or(0), 0])?; // transparent index, terminator
        Ok(())
    }

    /// Writes encoded image data sub-blocks.
    pub fn write_image_data(
        &mut self,
        descriptor: &ImageDescriptor,
        indices: &[u8],
        encoder: &mut LzwEncoder,
    ) -> Result<()> {
        // Image Descriptor
        self.writer.write_all(&[0x2C])?; // separator
        self.writer.write_all(&descriptor.x.to_le_bytes())?;
        self.writer.write_all(&descriptor.y.to_le_bytes())?;
        self.writer.write_all(&descriptor.width.to_le_bytes())?;
        self.writer.write_all(&descriptor.height.to_le_bytes())?;
        self.writer.write_all(&[0])?; // packed: no local palette

        // LZW Minimum Code Size
        self.writer.write_all(&[descriptor.lzw_min_code_size])?;

        // Encode data into sub-blocks
        let mut lzw_data = Vec::new();
        encoder.encode(indices, &mut lzw_data)?;

        // GIF requires data in sub-blocks (max 255 bytes)
        for chunk in lzw_data.chunks(255) {
            self.writer.write_all(&[chunk.len() as u8])?;
            self.writer.write_all(chunk)?;
        }
        self.writer.write_all(&[0])?; // block terminator

        Ok(())
    }

    /// Writes the GIF trailer byte.
    pub fn write_trailer(&mut self) -> Result<()> {
        self.writer.write_all(&[0x3B])?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_write_minimal_gif() {
        let mut buffer = Vec::new();
        {
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
            let descriptor = ImageDescriptor {
                x: 0,
                y: 0,
                width: 1,
                height: 1,
                lzw_min_code_size: 2,
            };
            writer.write_image_data(&descriptor, &[0], &mut encoder).unwrap();
            writer.write_trailer().unwrap();
        }

        assert!(buffer.starts_with(b"GIF89a"));
        assert_eq!(*buffer.last().unwrap(), 0x3B);
    }
}