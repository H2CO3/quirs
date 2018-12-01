//! The actual QR code decoder.

use std::ptr;
use std::usize;
use std::ffi::CStr;
use libc::c_int;
use geom::{ Image, QrCode };
use quirc_sys::{ quirc, quirc_version, quirc_new, quirc_destroy };
use quirc_sys::{ quirc_resize, quirc_begin, quirc_end };
use quirc_sys::{ quirc_code, quirc_count, quirc_extract };
use util::{ usize_to_int, int_to_usize };
use error::{ Error, Result };

/// A QR code decoder.
#[derive(Debug)]
pub struct Decoder {
    /// Opaque handle to the `quirc` decoder object.
    inner: *mut quirc,
}

impl Decoder {
    /// Attempts to create a `Decoder`.
    pub fn new() -> Result<Self> {
        let inner = unsafe { quirc_new() };

        if inner.is_null() {
            Err(Error::AllocFailed)
        } else {
            Ok(Decoder { inner })
        }
    }

    /// Return the version number of the `quirc` library, if possible.
    pub fn version() -> &'static str {
        let version_ptr = unsafe { quirc_version() };

        if version_ptr.is_null() {
            ""
        } else {
            unsafe {
                CStr::from_ptr(version_ptr).to_str().unwrap_or_default()
            }
        }
    }

    /// Feeds image data to the decoder and returns the QR codes.
    pub fn decode_image(&mut self, image: &Image) -> Result<Iter> {
        let width = usize_to_int(image.width())?;
        let height = usize_to_int(image.height())?;
        let image_data = image.data();

        unsafe {
            if quirc_resize(self.inner, width, height) != 0 {
                return Err(Error::AllocFailed);
            }

            let buf_ptr = quirc_begin(
                self.inner,
                ptr::null_mut(),
                ptr::null_mut(),
            );
            assert!(!buf_ptr.is_null(), "quirc_begin() returned null pointer");

            ptr::copy_nonoverlapping(
                image_data.as_ptr(),
                buf_ptr,
                image_data.len(),
            );

            quirc_end(self.inner);
        }

        Ok(Iter {
            decoder: self,
            index: 0,
        })
    }
}

impl Drop for Decoder {
    fn drop(&mut self) {
        unsafe {
            quirc_destroy(self.inner);
        }
    }
}

/// An iterator over QR codes in an image.
#[derive(Debug)]
pub struct Iter<'a> {
    /// A reference to the decoder where this iterator's contents come from.
    decoder: &'a mut Decoder,
    /// The index of the next image to process.
    index: c_int,
}

impl<'a> Iter<'a> {
    /// Returns the total count and the current index,
    /// ensuring that both are non-negative.
    fn count_and_index(&self) -> (c_int, c_int) {
        let count = unsafe {
            quirc_count(self.decoder.inner)
        };
        let index = self.index;

        assert!(count >= 0, "quirc_count() was negative");
        assert!(index >= 0, "current index was negative");

        (count, index)
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<QrCode>;

    fn next(&mut self) -> Option<Self::Item> {
        let (count, index) = self.count_and_index();

        if index < count {
            // This is not `mem::uninitialized` because `quirc_extract()`
            // returns without writing anything to the `quirc_code` out argument
            // if the index is OOB. Although we have a bounds check, I have
            // trust issues with underlying C libraries, so this remains a 0.
            let mut raw = quirc_code::default();

            unsafe {
                quirc_extract(self.decoder.inner, index, &mut raw);
            }

            self.index += 1;

            Some(QrCode::from_raw(raw))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let (count, index) = self.count_and_index();

        int_to_usize(count - index)
            .map(|n| (n, Some(n)))
            .unwrap_or_default() // we don't know if it under- or overflowed
    }
}
