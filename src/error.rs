//! Errors that can happen during QR code detection and decoding.

use std::fmt;
use std::error;
use std::result;
use quirc_sys::quirc_decode_error_t;

/// An error that could happen while using the `quirc` library.
#[derive(Debug, Clone, Copy)]
pub enum Error {
    /// Memory could not be allocated.
    AllocFailed,
    /// The length of the data buffer doesn't match the dimensions of the image.
    SizeMismatch,
    /// The size specified as a Rust `usize` can't be expressed in a C `int`
    /// or vice versa.
    IntOverflow,
    /// A decoding error occurred.
    DecodingFailed(DecodingErrorKind),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;

        self.description().fmt(f)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AllocFailed  => "memory allocation failed",
            Error::SizeMismatch => "buffer size doesn't match image dimensions",
            Error::IntOverflow  => "usize <-> int conversion would overflow",
            Error::DecodingFailed(reason) => reason.to_str(),
        }
    }
}

impl From<quirc_decode_error_t> for Error {
    fn from(error: quirc_decode_error_t) -> Self {
        Error::DecodingFailed(error.into())
    }
}

/// A decoding error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DecodingErrorKind {
    /// An unknown error happened.
    Unknown,
    /// The version was invalid.
    InvalidGridSize,
    /// The version was invalid.
    InvalidVersion,
    /// The format failed ECC check.
    FormatEcc,
    /// The data failed ECC check.
    DataEcc,
    /// The data type was not recognized.
    UnknownDataType,
    /// The data payload was too long.
    DataOverflow,
    /// The data payload was too short.
    DataUnderflow,
}

impl DecodingErrorKind {
    /// Returns a human-readable error message.
    pub fn to_str(self) -> &'static str {
        use self::DecodingErrorKind::*;

        match self {
            Unknown         => "decoding failed because an unknown error happened",
            InvalidGridSize => "decoding failed beacuse the grid size was invalid",
            InvalidVersion  => "decoding failed because the version was invalid",
            FormatEcc       => "decoding failed because the format failed ECC check",
            DataEcc         => "decoding failed because the data failed ECC check",
            UnknownDataType => "decoding failed because the data type was not recognized",
            DataOverflow    => "decoding failed because the data payload was too long",
            DataUnderflow   => "decoding failed because the data payload was too short",
        }
    }
}

impl From<quirc_decode_error_t> for DecodingErrorKind {
    fn from(code: quirc_decode_error_t) -> Self {
        use self::quirc_decode_error_t::*;
        use self::DecodingErrorKind::*;

        match code {
            QUIRC_ERROR_INVALID_GRID_SIZE => InvalidGridSize,
	        QUIRC_ERROR_INVALID_VERSION   => InvalidVersion,
	        QUIRC_ERROR_FORMAT_ECC        => FormatEcc,
	        QUIRC_ERROR_DATA_ECC          => DataEcc,
	        QUIRC_ERROR_UNKNOWN_DATA_TYPE => UnknownDataType,
	        QUIRC_ERROR_DATA_OVERFLOW     => DataOverflow,
	        QUIRC_ERROR_DATA_UNDERFLOW    => DataUnderflow,
            _                             => Unknown,
        }
    }
}

impl fmt::Display for DecodingErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.to_str())
    }
}

/// A `Result` which may hold a Qui-RS `Error`.
pub type Result<T> = result::Result<T, Error>;
