use std::env;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::process::exit;

mod image;

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);
    let target: Box<dyn Write> = if let Some(out) = args.next() {
        match fs::OpenOptions::new().write(true).open(out) {
            Ok(file) => Box::new(file),
            Err(err) => {
                eprintln!("{}", err);
                exit(1)
            }
        }
    } else {
        Box::new(io::stdout())
    };
    let mut writer = BufWriter::new(target);

    // Image
    let width: u32 = 501;
    let height: u32 = 501;

    // Note: (0, 0) is the bottom-left corner
    // (x, 0) is the x/hor axis 
    // (0, y) is the y/ver axis

    // Render
    // BMP File Header (14 bytes)
    writer.write(b"BM")?;                                       // Signature
    writer.write(&u32::to_le_bytes(54 + width * height * 3))?;  // File size: (headers=54) + pixels
    writer.write(&[0; 4])?;                                     // Reserved: (set to 0)
    writer.write(&u32::to_le_bytes(54))?;                       // Offset to pixel data

    // DIB Header (40 bytes)
    writer.write(&u32::to_le_bytes(40))?;                   // Size of DIB Header: 40 for BITMAPINFOHEADER
    writer.write(&i32::try_from(width)
        .expect("Image dimensions must be <= i32::MAX")     // Width (signed)
        .to_le_bytes())?;
    writer.write(&i32::try_from(width)                      // Height (signed)
        .expect("Image dimensions must be <= i32::MAX")
        .to_le_bytes())?;
    writer.write(&u16::to_le_bytes(1))?;                    // Number of color panes: always 1
    writer.write(&u16::to_le_bytes(24))?;                   // Number of bits per pixel: 24=1 byte per color
    writer.write(&u32::to_le_bytes(0))?;                    // Compression: 0=no compression
    writer.write(&u32::to_le_bytes(0))?;                    // Raw image size: 0=uncompressed
    writer.write(&i32::to_le_bytes(0))?;                    // Horizontal resolution: pixel/m (signed)
    writer.write(&i32::to_le_bytes(0))?;                    // Vertical resolution: pixel/m (signed)
    writer.write(&u32::to_le_bytes(0))?;                    // Number of colors: 0=2^n
    writer.write(&u32::to_le_bytes(0))?;                    // Number of important colors: 0=all important

    // Pixel Data
    for j in 0..height {
        for i in 0..width {
            writer.write(&[(256.0 * i as f64 / (width as f64 - 1.0)) as u8])?;
            writer.write(&[(256.0 * j as f64/ (height as f64 - 1.0)) as u8])?;
            writer.write(&[0])?;
        }

        // Pad row to a multiple of 4 bytes
        match (width * 3) % 4 {
            0 => 0,
            1 => writer.write(&[0, 0, 0])?,
            2 => writer.write(&[0, 0])?,
            3 => writer.write(&[0])?,
            _ => unreachable!()
        };
    }

    writer.flush()?;
    return Ok(());
}
