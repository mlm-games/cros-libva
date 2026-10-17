// Stub for non-Linux/Android targets (wasm, windows, macos)
// Provides minimal API so your arch matrix (x86_64/aarch64/wasm/android)
// like `clipawl`/`repose` still `cargo check` without libva.
// Real VA-API is Linux(+Android via cargo-rapk) only.

use std::sync::Arc;

#[derive(Debug)]
pub struct VaError(pub i32);
impl std::fmt::Display for VaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VA-API not available on this platform (stub)")
    }
}
impl std::error::Error for VaError {}

pub struct Display;
impl Display {
    pub fn open() -> Option<Arc<Self>> {
        None
    }
    pub fn open_drm_display<P: AsRef<std::path::Path>>(
        _path: P,
    ) -> Result<Arc<Self>, std::io::Error> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "VA-API not available on this platform",
        ))
    }
}

// Minimal re-exports so `use libva::*` doesn't break on wasm/windows.
pub type VAStatus = i32;
pub const VA_STATUS_SUCCESS: i32 = 0;
pub type VADisplay = *mut std::os::raw::c_void;
pub type VASurfaceID = u32;
pub type VAConfigID = u32;
pub type VAContextID = u32;
pub type VABufferID = u32;
pub use std::os::raw::c_void as _c_void;

pub struct UsageHint;
impl UsageHint {
    pub const USAGE_HINT_DECODER: Self = Self;
    pub const USAGE_HINT_ENCODER: Self = Self;
}
