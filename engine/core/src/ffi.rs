use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

use crate::engine::LauncherEngine;

#[no_mangle]
pub extern "C" fn a_launcher_native_launch(
    storage_root: *const c_char,
    instance_id: *const c_char,
) -> c_int {
    if storage_root.is_null() || instance_id.is_null() {
        return 0;
    }

    let root = unsafe { CStr::from_ptr(storage_root) };
    let id = unsafe { CStr::from_ptr(instance_id) };

    let Ok(root) = root.to_str() else { return 0; };
    let Ok(id) = id.to_str() else { return 0; };
    if id.trim().is_empty() || root.trim().is_empty() {
        return 0;
    }

    match LauncherEngine::new(root) {
        Ok(engine) => match engine.instance_launch_config(id) {
            Ok(_) => 1,
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}
