//! Basic geometry types.

use std::fmt;
use util::int_to_usize;
use info::Info;
use quirc_sys::{ quirc_point, quirc_code, quirc_data };
use quirc_sys::{ quirc_decode, quirc_decode_error_t };
use error::{ Error, Result };
use self::quirc_decode_error_t::QUIRC_SUCCESS;

/// A size, offset, or point in the 2-dimensional plane.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Vec2D {
    /// X coordinate, horizontal position, or width.
    pub x: usize,
    /// Y coordinate, vertical position, or height.
    pub y: usize,
}

impl Vec2D {
    /// Attempt to convert a `quirc_point` to a `Vec2D` without over- or underflow.
    pub fn from_raw(p: quirc_point) -> Result<Self> {
        let (x, y) = (int_to_usize(p.x)?, int_to_usize(p.y)?);
        Ok(Vec2D { x, y })
    }
}

/// Raw image data to be decoded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Image<'a> {
    /// The data itself as a contiguous slice.
    data: &'a [u8],
    /// The dimensions of the image.
    size: Vec2D,
}

impl<'a> Image<'a> {
    /// Creates a image out of a raw buffer of grayscale data,
    /// and the width and the height of the image.
    pub fn new(data: &'a [u8], size: Vec2D) -> Result<Self> {
        if data.len() == size.x * size.y {
            Ok(Image { data, size })
        } else {
            Err(Error::SizeMismatch)
        }
    }

    /// Return the raw data buffer.
    pub fn data(&self) -> &[u8] {
        self.data
    }

    /// Return the width of (number of columns in) the image.
    pub fn width(&self) -> usize {
        self.size.x
    }

    /// Return the height of (number of rows in) the image.
    pub fn height(&self) -> usize {
        self.size.y
    }
}

/// Information about the location and raw data of a QR code within an `Image`.
#[derive(Clone, Copy)]
pub struct QrCode(quirc_code);

impl QrCode {
    /// Attempts to create a new `QrCode` from a `quirc_code` FFI struct.
    #[doc(hidden)]
    pub fn from_raw(raw: quirc_code) -> Result<Self> {
        let _ = int_to_usize(raw.size)?;

        for i in 0..4 {
            let _ = Vec2D::from_raw(raw.corners[i])?;
        }

        Ok(QrCode(raw))
    }

    /// Extracts the corner at the given index (0...3) as a `Vec2D`.
    fn corner_at(&self, i: usize) -> Vec2D {
        // This cannot panic because before the construction of the `QrCode`,
        // the underlying `quirc_code` is validated for representability as
        // Rust types (underflow and overflow of `usize` etc.)
        Vec2D::from_raw(self.0.corners[i]).expect("invalid corner coordinates")
    }

    /// The coordinates of the top left corner of the QR code.
    pub fn top_left_corner(&self) -> Vec2D {
        self.corner_at(0)
    }

    /// The coordinates of the top right corner of the QR code.
    pub fn top_right_corner(&self) -> Vec2D {
        self.corner_at(1)
    }

    /// The coordinates of the bottom right corner of the QR code.
    pub fn bottom_right_corner(&self) -> Vec2D {
        self.corner_at(2)
    }

    /// The coordinates of the bottom left corner of the QR code.
    pub fn bottom_left_corner(&self) -> Vec2D {
        self.corner_at(3)
    }

    /// The size (`width == height`) of the QR code bitmap.
    pub fn size(&self) -> usize {
        // This cannot panic because before the construction of the `QrCode`,
        // the underlying `quirc_code` is validated for representability as
        // Rust types (underflow and overflow of `usize` etc.)
        int_to_usize(self.0.size).expect("code size under- or overflows usize")
    }

    /// A reference to the bitmap buffer.
    /// If `size * size % 8 != 0`, then not all bits of the last byte returned
    /// correspond to any actual bitmap data.
    pub fn bitmap(&self) -> &[u8] {
        let size = self.size();
        let num_bits = size * size;
        let num_bytes = (num_bits + 7) / 8;
        &self.0.cell_bitmap[..num_bytes]
    }

    /// Get the bit at coordinates `(coord.x, coord.y)`,
    /// performing a bounds check.
    pub fn get(&self, coord: Vec2D) -> Option<bool> {
        let size = self.size();
        let Vec2D { x, y } = coord;

        if x < size && y < size {
            let i = y * size + x;
            let bit = self.0.cell_bitmap[i / 8] >> (i % 8) & 1;
            Some(bit != 0)
        } else {
            None
        }
    }

    /// Get the bit at coordinates `(coord.x, coord.y)`,
    /// panicking upon an OOB condition.
    pub fn bit_at(&self, coord: Vec2D) -> bool {
        self.get(coord).unwrap_or_else(
            || panic!("{:?} out of bounds for bitmap of size {}", coord, self.size())
        )
    }

    /// Decode the raw data into higher-level information.
    pub fn decode(&self) -> Result<Info> {
        let mut raw = quirc_data::default();
        let error_code = unsafe {
            quirc_decode(&self.0, &mut raw)
        };

        if error_code == QUIRC_SUCCESS {
            Ok(Info::from_raw(raw))
        } else {
            Err(error_code.into())
        }
    }
}

impl fmt::Debug for QrCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("QrCode")
            .field("top_left_corner", &self.top_left_corner())
            .field("top_right_corner", &self.top_right_corner())
            .field("bottom_right_corner", &self.bottom_right_corner())
            .field("bottom_left_corner", &self.bottom_left_corner())
            .field("size", &self.size())
            .field("bitmap", &self.bitmap())
            .finish()
    }
}
