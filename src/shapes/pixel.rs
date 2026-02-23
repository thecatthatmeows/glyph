use std::io::{StdoutLock, Write};

use crossterm::{cursor::MoveTo, queue, style::{Color, Print, SetForegroundColor}};

/// A single raster pixel produced by shapes when rasterizing into a batch.
///
/// `z` is the z-index for that pixel (lower is drawn first). `x` and `y` are
/// terminal coordinates (u16) suitable for `crossterm::cursor::MoveTo`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pixel {
    pub x: u16,
    pub y: u16,
    pub ch: char,
    pub color: Color,
    pub z: i32,
}

impl Pixel {
    pub fn new(x: u16, y: u16, ch: char, color: Color, z: i32) -> Self {
        Self { x, y, ch, color, z }
    }
}

/// Flush a batch of pixels to a locked stdout in a single, ordered pass.
///
/// This helper will:
/// - sort pixels by (z, y, x) so that z-index ordering is respected and output is
///   mostly row-major for better grouping,
/// - minimize `SetForegroundColor` calls by only issuing a new color command
///   when it changes,
/// - perform a single `flush()` at the end and clear the pixel buffer.
pub fn flush_pixels(stdout: &mut StdoutLock<'_>, pixels: &mut Vec<Pixel>) {
    if pixels.is_empty() {
        stdout.flush().ok();
        return;
    }

    // Sort by z (asc), then y (asc), then x (asc) to respect z-index and draw
    // in a mostly row-major order for better write locality.
    pixels.sort_by_key(|p| (p.z, p.y, p.x));

    let mut last_color: Option<Color> = None;
    for p in pixels.iter() {
        if last_color != Some(p.color) {
            queue!(stdout, SetForegroundColor(p.color)).unwrap();
            last_color = Some(p.color);
        }
        queue!(stdout, MoveTo(p.x, p.y), Print(p.ch)).unwrap();
    }

    stdout.flush().unwrap();
    pixels.clear();
}
