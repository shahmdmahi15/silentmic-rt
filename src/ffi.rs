//! C Foreign Function Interface (FFI) bindings for SilentMic-RT.
//!
//! Provides a pure C-compatible ABI for integrating SilentMic-RT into C, C++, Python,
//! C#, Swift, Go, and OBS Studio plugins.

use std::ffi::c_char;
use std::ptr;
use crate::engine::SilentMicEngine;
use crate::dsp::stft::HOP_SIZE;

/// Opaque pointer handle representing a SilentMic engine instance in C.
pub type SilentMicHandle = *mut SilentMicEngine;

/// C-compatible engine statistics struct.
#[repr(C)]
pub struct SilentMicStatsC {
    pub frames_processed: u64,
    pub samples_processed: u64,
    pub last_frame_micros: f64,
    pub max_frame_micros: f64,
    pub avg_frame_micros: f64,
    pub cpu_budget_pct: f64,
}

/// Creates a new SilentMic-RT engine instance.
///
/// # Arguments
/// * `sample_rate` - Must be 48000 Hz.
///
/// Returns non-null pointer on success, or NULL if sample rate is unsupported.
#[no_mangle]
pub unsafe extern "C" fn silentmic_create(sample_rate: u32) -> SilentMicHandle {
    if sample_rate != 48000 {
        return ptr::null_mut();
    }

    let engine = Box::new(SilentMicEngine::new());
    Box::into_raw(engine)
}

/// Processes a single 64-sample audio frame in real time.
///
/// # Arguments
/// * `handle` - Valid pointer returned by `silentmic_create`.
/// * `in_frame` - Pointer to 64 consecutive float32 samples.
/// * `out_frame` - Pointer to output buffer of at least 64 float32 samples.
///
/// Returns 0 on success, -1 on null pointer.
#[no_mangle]
pub unsafe extern "C" fn silentmic_process_frame(
    handle: SilentMicHandle,
    in_frame: *const f32,
    out_frame: *mut f32,
) -> i32 {
    if handle.is_null() || in_frame.is_null() || out_frame.is_null() {
        return -1;
    }

    let engine = &mut *handle;
    let in_slice = std::slice::from_raw_parts(in_frame, HOP_SIZE);
    let out_slice = std::slice::from_raw_parts_mut(out_frame, HOP_SIZE);

    let mut in_arr = [0.0f32; HOP_SIZE];
    let mut out_arr = [0.0f32; HOP_SIZE];
    in_arr.copy_from_slice(in_slice);

    engine.process_frame(&in_arr, &mut out_arr);
    out_slice.copy_from_slice(&out_arr);

    0
}

/// Sets maximum noise floor attenuation in decibels (e.g. -48.0).
#[no_mangle]
pub unsafe extern "C" fn silentmic_set_attenuation_db(handle: SilentMicHandle, db: f32) -> i32 {
    if handle.is_null() {
        return -1;
    }
    let engine = &mut *handle;
    engine.set_attenuation_db(db);
    0
}

/// Resets internal delay lines and recurrent state.
#[no_mangle]
pub unsafe extern "C" fn silentmic_reset(handle: SilentMicHandle) -> i32 {
    if handle.is_null() {
        return -1;
    }
    let engine = &mut *handle;
    engine.reset();
    0
}

/// Retrieves real-time performance statistics.
#[no_mangle]
pub unsafe extern "C" fn silentmic_get_stats(handle: SilentMicHandle, stats_out: *mut SilentMicStatsC) -> i32 {
    if handle.is_null() || stats_out.is_null() {
        return -1;
    }
    let engine = &*handle;
    let s = engine.stats();
    *stats_out = SilentMicStatsC {
        frames_processed: s.frames_processed,
        samples_processed: s.samples_processed,
        last_frame_micros: s.last_frame_micros,
        max_frame_micros: s.max_frame_micros,
        avg_frame_micros: s.avg_frame_micros,
        cpu_budget_pct: s.cpu_budget_pct,
    };
    0
}

/// Frees an engine instance previously created by `silentmic_create`.
#[no_mangle]
pub unsafe extern "C" fn silentmic_destroy(handle: SilentMicHandle) {
    if !handle.is_null() {
        drop(Box::from_raw(handle));
    }
}

/// Returns the semver version string of the SilentMic-RT library.
#[no_mangle]
pub extern "C" fn silentmic_version() -> *const c_char {
    static VERSION: &[u8] = b"0.1.0\0";
    VERSION.as_ptr() as *const c_char
}
