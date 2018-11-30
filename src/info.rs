//! High-level representation of the information contained in a QR code.

use std::str::{ self, Utf8Error };
use std::cmp::{ min, max };
use std::hash::{ Hash, Hasher };
use quirc_sys::{ quirc_data, QUIRC_MAX_PAYLOAD };
use quirc_sys::QuircEccLevel::*;
use quirc_sys::QuircDataType::*;

/// High-level representation of the information contained in a QR code.
#[derive(Debug, Clone, Copy)]
pub struct Info(quirc_data);

impl Info {
    /// Attempts to extract high-level information from the raw FFI `quirc_data`.
    #[doc(hidden)]
    pub fn from_raw(raw: quirc_data) -> Self {
        Info(raw)
    }

    /// Returns the version number of the code, in the range `1...40`.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation, cast_possible_wrap))]
    pub fn version(&self) -> u8 {
        max(1, min(self.0.version, 40)) as _
    }

    /// Returns the mask ID of the code, in the range `0...7`.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation, cast_possible_wrap))]
    pub fn mask_id(&self) -> u8 {
        max(0, min(self.0.mask, 7)) as _
    }

    /// Returns the ECI assignment number, in the range 0...30.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation, cast_possible_wrap))]
    pub fn eci(&self) -> u8 {
        max(0, min(self.0.eci, 30)) as _
    }

    /// Returns the error correction level of the code.
    pub fn ecc_level(&self) -> EccLevel {
        let ecc = self.0.ecc_level;

        // Casting an `int` with a value that isn't valid for a Rust `enum` is
        // Undefined Behavior, so we must perform the conversion in the opposite
        // direction. Therefore, we can't use `match`.
        if ecc == QUIRC_ECC_LEVEL_L as _ {
            EccLevel::L
        } else if ecc == QUIRC_ECC_LEVEL_M as _ {
            EccLevel::M
        } else if ecc == QUIRC_ECC_LEVEL_Q as _ {
            EccLevel::Q
        } else if ecc == QUIRC_ECC_LEVEL_H as _ {
            EccLevel::H
        } else {
            EccLevel::L // assume only the lowest level of ECC for robustness
        }
    }

    /// Returns the highest-valued data type found in the code.
    pub fn data_type(&self) -> DataType {
        // For the rationale behind this implementation,
        // see the comment in `ecc_level()` above.
        let dtype = self.0.data_type;

        if dtype == QUIRC_DATA_TYPE_NUMERIC as _ {
            DataType::Numeric
        } else if dtype == QUIRC_DATA_TYPE_ALPHA as _ {
            DataType::Alphanumeric
        } else if dtype == QUIRC_DATA_TYPE_BYTE as _ {
            DataType::Byte
        } else if dtype == QUIRC_DATA_TYPE_KANJI as _ {
            DataType::Kanji
        } else {
            DataType::Byte // assume uninterpreted raw bytes if type is unknown
        }
    }

    /// Returns the raw payload of the QR code.
    pub fn payload(&self) -> &[u8] {
        let len = max(0, min(self.0.payload_len as _, QUIRC_MAX_PAYLOAD));
        // We statically know that `QUIRC_MAX_PAYLOAD` fits into a `usize`,
        // which is always at least 16 bits long in Rust.
        // Hence, this cast is safe.
        &self.0.payload[..len]
    }

    /// Returns the payload as UTF-8 text if possible.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        str::from_utf8(self.payload())
    }
}

impl PartialEq<Info> for Info {
    fn eq(&self, other: &Info) -> bool {
        self.version() == other.version() &&
        self.mask_id() == other.mask_id() &&
        self.eci() == other.eci() &&
        self.ecc_level() == other.ecc_level() &&
        self.data_type() == other.data_type() &&
        self.payload() == other.payload()
    }
}

impl Hash for Info {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.version().hash(state);
        self.mask_id().hash(state);
        self.eci().hash(state);
        self.ecc_level().hash(state);
        self.data_type().hash(state);
        self.payload().hash(state);
    }
}

/// The ECC level used for the QR code.
///
/// NB: deriving `PartialOrd` and `Ord` produces the correct ordering even
/// though the numerical values of the variants aren't in monotonic order.
/// This is because `derive` takes into account the order in which the
/// variants have been declared, and not their raw integer value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EccLevel {
    /// Low error correction: ~7% loss recoverable.
    L = QUIRC_ECC_LEVEL_L as _,
    /// Medium error correction: ~15% loss recoverable.
    M = QUIRC_ECC_LEVEL_M as _,
    /// Quartile error correction: ~25% loss recoverable.
    Q = QUIRC_ECC_LEVEL_Q as _,
    /// High error correction: ~30% loss recoverable.
    H = QUIRC_ECC_LEVEL_H as _,
}

/// The highest-valued (most complex) data type found in the QR code.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DataType {
    /// Numeric (digits only).
    Numeric      = QUIRC_DATA_TYPE_NUMERIC as _,
    /// Alphanumeric.
    Alphanumeric = QUIRC_DATA_TYPE_ALPHA   as _,
    /// Bytes.
    Byte         = QUIRC_DATA_TYPE_BYTE    as _,
    /// Kanji characters.
    Kanji        = QUIRC_DATA_TYPE_KANJI   as _,
}
