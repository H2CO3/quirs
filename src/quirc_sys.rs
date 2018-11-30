//! Raw FFI bindings for the `quirc` C API.

use std::fmt;
use libc::{ c_int, c_char };

/// Opaque type manipulated by the `quirc` C API.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug)]
pub struct quirc {
    /// Just a zero-sized, `#[repr(C)]`-compatible private field serving as a
    /// guard against instantiating the opaque type from the outside world.
    _dummy: [u8; 0],
}

/// This structure describes a location in the input image buffer.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct quirc_point {
    /// X coordinate (column) of the point.
	pub x: c_int,
    /// Y coordinate (row) of the point.
	pub y: c_int,
}

/// This structure is used to return information about detected QR codes
/// in the input image.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct quirc_code {
	/// The four corners of the QR-code, from top left, clockwise */
	pub corners: [quirc_point; 4],
	/// The number of cells across in the QR-code. The cell bitmap
	/// is a bitmask giving the actual values of cells. If the cell
	/// at (x, y) is black, then the following bit is set:
    ///
	///     cell_bitmap[i >> 3] & (1 << (i & 7))
    ///
	/// where i = (y * size) + x.
	pub size: c_int,
    /// The actual bitmap data.
	pub cell_bitmap: [u8; QUIRC_MAX_BITMAP],
}

impl Default for quirc_code {
    fn default() -> Self {
        quirc_code {
            corners: Default::default(),
            size: 0,
            cell_bitmap: [0; QUIRC_MAX_BITMAP],
        }
    }
}

impl fmt::Debug for quirc_code {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("quirc_code")
            .field("corners", &self.corners)
            .field("size", &self.size)
            .field("cell_bitmap", &self.cell_bitmap.as_ref())
            .finish()
    }
}

/// This structure holds the decoded QR-code data.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Clone, Copy)]
pub struct quirc_data {
	/// Various parameters of the QR-code. These can mostly be
	/// ignored if you only care about the data.
    /// Version number.
	pub version: c_int,
    /// Error-correction level.
	pub ecc_level: c_int,
    /// Mask.
	pub mask: c_int,
	/// This field is the highest-valued data type found in the QR code.
	pub data_type: c_int,
	/// Data payload. For the Kanji datatype, payload is encoded as
	/// Shift-JIS. For all other datatypes, payload is ASCII text.
	pub payload: [u8; QUIRC_MAX_PAYLOAD],
    /// Valid (initialized) portion of `payload`.
	pub payload_len: c_int,
	/// ECI assignment number.
	pub eci: u32,
}

impl Default for quirc_data {
    /// I don't particularly care that these default values don't make much sense.
    /// They won't be used outside the library anyway since this module is private,
    /// and the values are checked before conversion to a public type, so no UB
    /// can occur even if e.g. the value 0 is invalid for a certain `enum`.
    /// But I want to ensure I have initialized memory when working with a C library,
    /// and providing zeroed memory is standard practice for that.
    fn default() -> Self {
        quirc_data {
            version: 0,
            ecc_level: 0,
            mask: 0,
            data_type: 0,
            payload: [0; QUIRC_MAX_PAYLOAD],
            payload_len: 0,
            eci: 0,
        }
    }
}

impl fmt::Debug for quirc_data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("quirc_data")
            .field("version", &self.version)
            .field("ecc_level", &self.ecc_level)
            .field("mask", &self.mask)
            .field("data_type", &self.data_type)
            .field("payload", &self.payload.as_ref())
            .field("payload_len", &self.payload_len)
            .field("eci", &self.eci)
            .finish()
    }
}

/// QR-code ECC types.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuircEccLevel {
    /// Medium ECC: correct at most 15% damage.
    QUIRC_ECC_LEVEL_M = 0,
    /// Low ECC: correct at most 7% damage.
    QUIRC_ECC_LEVEL_L = 1,
    /// High ECC: correct at most 30% damage.
    QUIRC_ECC_LEVEL_H = 2,
    /// Quartile ECC: correct at most 25% damage.
    QUIRC_ECC_LEVEL_Q = 3,
}

/// QR-code data types.
#[allow(non_camel_case_types)]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuircDataType {
    /// Numeric (digits only).
    QUIRC_DATA_TYPE_NUMERIC = 1,
    /// Alphanumeric.
    QUIRC_DATA_TYPE_ALPHA   = 2,
    /// Bytes.
    QUIRC_DATA_TYPE_BYTE    = 4,
    /// Kanji characters.
    QUIRC_DATA_TYPE_KANJI   = 8,
}

/// This enum describes the various decoder errors which may occur.
#[repr(C)]
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum quirc_decode_error_t {
    /// No error.
	QUIRC_SUCCESS = 0,
    /// Invalid grid size.
	QUIRC_ERROR_INVALID_GRID_SIZE,
    /// Invalid version.
	QUIRC_ERROR_INVALID_VERSION,
    /// Invalid ECC format.
	QUIRC_ERROR_FORMAT_ECC,
    /// ECC data error.
	QUIRC_ERROR_DATA_ECC,
    /// Invalid data type.
	QUIRC_ERROR_UNKNOWN_DATA_TYPE,
    /// Data overflow.
	QUIRC_ERROR_DATA_OVERFLOW,
    /// Data underflow.
	QUIRC_ERROR_DATA_UNDERFLOW,
}

/// Limits on the maximum size of QR-codes and their content.
pub const QUIRC_MAX_BITMAP:  usize = 3917;
/// Limits on the maximum size of QR-codes and their content.
pub const QUIRC_MAX_PAYLOAD: usize = 8896;

extern {
    /// Obtain the library version string.
    pub fn quirc_version() -> *const c_char;

    /// Construct a new QR-code recognizer. This function will return NULL
    /// if sufficient memory could not be allocated.
    pub fn quirc_new() -> *mut quirc;

    /// Destroy a QR-code recognizer.
    pub fn quirc_destroy(q: *mut quirc);

    /// Resize the QR-code recognizer. The size of an image must be
    /// specified before codes can be analyzed.
    ///
    /// This function returns 0 on success, or -1 if sufficient memory could
    /// not be allocated.
    pub fn quirc_resize(q: *mut quirc, w: c_int, h: c_int) -> c_int;

    /// These functions are used to process images for QR-code recognition.
    /// `quirc_begin()` must first be called to obtain access to a buffer into
    /// which the input image should be placed. Optionally, the current
    /// width and height may be returned.
    pub fn quirc_begin(q: *mut quirc, q: *mut c_int, h: *mut c_int) -> *mut u8;

    /// After filling the buffer, `quirc_end()` should be called to process
    /// the image for QR-code recognition. The locations and content of each
    /// code may be obtained using accessor functions described below.
    pub fn quirc_end(q: *mut quirc);

    /// Return the number of QR-codes identified in the last processed image.
    pub fn quirc_count(q: *const quirc) -> c_int;

    /// Extract the QR-code specified by the given index.
    pub fn quirc_extract(q: *const quirc, index: c_int, code: *mut quirc_code);

    /// Decode a QR-code, returning the payload data.
    pub fn quirc_decode(code: *const quirc_code,
                        data: *mut quirc_data) -> quirc_decode_error_t;
}
