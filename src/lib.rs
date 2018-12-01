//! Wrapper around the `quirc` QR code decoder library.

#![crate_name="quirs"]
#![doc(html_root_url = "https://docs.rs/quirs/0.1.1")]
#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications, missing_docs)]
#![cfg_attr(feature = "cargo-clippy",
            allow(single_match, match_same_arms, match_ref_pats,
                  clone_on_ref_ptr, needless_pass_by_value))]
#![cfg_attr(feature = "cargo-clippy",
            deny(wrong_pub_self_convention, used_underscore_binding,
                 stutter, similar_names, pub_enum_variant_names,
                 missing_docs_in_private_items,
                 non_ascii_literal, unicode_not_nfc,
                 result_unwrap_used, option_unwrap_used,
                 option_map_unwrap_or_else, option_map_unwrap_or, filter_map,
                 shadow_unrelated, shadow_reuse, shadow_same,
                 int_plus_one, string_add_assign, if_not_else,
                 invalid_upcast_comparisons,
                 cast_precision_loss, cast_lossless,
                 cast_possible_wrap, cast_possible_truncation,
                 mutex_integer, mut_mut, items_after_statements,
                 print_stdout, mem_forget, maybe_infinite_iter))]

extern crate libc;

mod quirc_sys;
mod util;

pub mod decoder;
pub mod info;
pub mod geom;
pub mod error;

pub use decoder::Decoder;
pub use error::Error;
pub use geom::{ Image, Vec2D, QrCode };
pub use info::Info;
