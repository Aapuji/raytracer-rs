use std::env;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::process::exit;

use color::{write_color, Color};
use image::write_header;

mod color;
mod image;
mod ray;
mod vec3;

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
    let width: u32 = 256;
    let height: u32 = 256;

    // Note:
    // (0, 0) is the bottom-left corner
    // (x, 0) is the x/hor axis
    // (0, y) is the y/ver axis

    // Render
    write_header(&mut writer, width, height)?;

    // Pixel Data
    for j in 0..height {
        #[cfg(debug_assertions)]
        {
            println!("Scanlines remaining: {}", (height - j));
        }

        for i in 0..width {
            let color = Color::new(
                0.0,
                j as f64 / (height - 1) as f64,
                i as f64 / (width - 1) as f64,
            );
            write_color(&mut writer, &color)?;
        }

        // Pad row to a multiple of 4 bytes
        match (width * 3) % 4 {
            0 => 0,
            1 => writer.write(&[0, 0, 0])?,
            2 => writer.write(&[0, 0])?,
            3 => writer.write(&[0])?,
            _ => unreachable!(),
        };
    }

    #[cfg(debug_assertions)]
    {
        println!("Done");
    }

    writer.flush()?;
    return Ok(());
}
