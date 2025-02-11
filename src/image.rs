use std::io::{self, BufWriter, Write};

#[rustfmt::skip]
/// Writes the required BMP file header (using the `BITMAPINFOHEADER` format) given the dimensions of the image into the given `writer`.
///
/// Does not flush the `writer`.
/// Does not check if `width` and `height` are less than `i32::MAX` (will panic if they are not).
/// If any of the write operations error, they will propagate out the function and the remaining
/// writes will not occur.
pub fn write_header<W: Write>(
    writer: &mut BufWriter<W>,
    width: u32,
    height: u32,
) -> io::Result<()> {
    // BMP File Header (14 bytes)
    writer.write(b"BM")?;                                       // Signature
    writer.write(&u32::to_le_bytes(54 + width * height * 3))?;  // File size: (headers=54) + pixels
    writer.write(&[0; 4])?;                                     // Reserved: (set to 0)
    writer.write(&u32::to_le_bytes(54))?;                       // Offset to pixel data

    // DIB Header (40 bytes)
    writer.write(&u32::to_le_bytes(40))?;                   // Size of DIB Header: 40 for BITMAPINFOHEADER
    writer.write(                                           // Width (signed)
        &i32::try_from(width)
            .expect("Image dimensions must be <= i32::MAX")
            .to_le_bytes(),
    )?;
    writer.write(                                           // Height (signed)
        &i32::try_from(height)
            .expect("Image dimensions must be <= i32::MAX")
            .to_le_bytes(),
    )?;
    writer.write(&u16::to_le_bytes(1))?;                    // Number of color panes: always 1
    writer.write(&u16::to_le_bytes(24))?;                   // Number of bits per pixel: 24=1 byte per color
    writer.write(&u32::to_le_bytes(0))?;                    // Compression: 0=no compression
    writer.write(&u32::to_le_bytes(0))?;                    // Raw image size: 0=uncompressed
    writer.write(&i32::to_le_bytes(0))?;                    // Horizontal resolution: pixel/m (signed)
    writer.write(&i32::to_le_bytes(0))?;                    // Vertical resolution: pixel/m (signed)
    writer.write(&u32::to_le_bytes(0))?;                    // Number of colors: 0=2^n
    writer.write(&u32::to_le_bytes(0))?;                    // Number of important colors: 0=all important

    return Ok(());
}
