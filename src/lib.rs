//! Rust bindings for meshoptimizer/gltfpack
//!
//! This crate provides bindings to [meshoptimizer](https://github.com/zeux/meshoptimizer)'s
//! gltfpack functionality for compressing glTF/GLB files.
//!
//! ## Attribution
//!
//! This crate bundles meshoptimizer v0.25 by Arseny Kapoulkine, licensed under MIT.
//! See the NOTICE file for full license text.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi::CString;
use std::os::raw::{c_char, c_int};
use std::path::Path;

extern "C" {
    #[link_name = "compress"]
    fn compress_ffi(input: *const c_char, output: *const c_char) -> c_int;
}

/// Compress a glTF/GLB file with maximum compression (equivalent to gltfpack -cc)
///
/// # Arguments
/// * `input` - Path to input file (.obj/.gltf/.glb)
/// * `output` - Path to output file (.gltf/.glb)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(i32)` with error code on failure
///
/// # Example
/// ```no_run
/// use gltfpack_sys::compress;
///
/// compress("model.glb", "model_compressed.glb").unwrap();
/// ```
pub fn compress(input: impl AsRef<Path>, output: impl AsRef<Path>) -> Result<(), i32> {
    let input_c = CString::new(input.as_ref().to_str().ok_or(-1)?).map_err(|_| -1)?;
    let output_c = CString::new(output.as_ref().to_str().ok_or(-1)?).map_err(|_| -1)?;

    let result = unsafe { compress_ffi(input_c.as_ptr(), output_c.as_ptr()) };

    if result == 0 {
        Ok(())
    } else {
        Err(result)
    }
}
