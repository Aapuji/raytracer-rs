use std::io::{self, BufWriter, Write};

use crate::vec3::Vec3;

/// A `Color` represents a color stored in RGB format as a `Vec3` of floating points, so that
/// calculations can be done on the values.
/// When they are written, then they are transformed back to `u8`.
pub type Color = Vec3;

/// Writes the rgb ratios in `color` to the `writer`. Values of `color` should be between 0 and 1.
pub fn write_color<W: Write>(writer: &mut BufWriter<W>, color: &Color) -> io::Result<()> {
    // BMP files use BGR format

    writer.write(&[(256.0 * color.z()) as u8])?; // Blue
    writer.write(&[(256.0 * color.y()) as u8])?; // Green
    writer.write(&[(256.0 * color.x()) as u8])?; // Red

    Ok(())
}
