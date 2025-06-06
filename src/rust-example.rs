#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "C" fn Halon_version(
) -> u32 {
    HALONMTA_PLUGIN_VERSION
}

use std::{
    ffi::CStr,
    ffi::c_void,
    ffi::c_char,
    ptr::null_mut
};

pub extern "C" fn rust_example(
    _hhc: *mut HalonHSLContext,
    _args: *mut HalonHSLArguments,
    ret: *mut HalonHSLValue,
) {
    unsafe {
        // get first argument
        let arg = HalonMTA_hsl_argument_get(_args, 0);
        if arg == null_mut() {
            return
        }

        // convert to a string
        let mut input: *mut c_char = null_mut();
        let input_ptr: *mut *mut c_char = &mut input;
        let ok = HalonMTA_hsl_value_get(arg, HALONMTA_HSL_TYPE_STRING as i32, input_ptr as *mut c_void, null_mut());
        if !ok {
            libc::free(input as *mut c_void);
        }
        let input_cstr: &CStr = CStr::from_ptr(input);
        let input_str = String::from_utf8_lossy(input_cstr.to_bytes()).to_string();

        // set as return value
        let output = std::ffi::CString::new(input_str).unwrap();
        HalonMTA_hsl_value_set(ret, HALONMTA_HSL_TYPE_STRING as i32, output.as_ptr() as *mut std::ffi::c_void, 0);
    }
}

#[no_mangle]
pub extern "C" fn Halon_hsl_register(hhrc: *mut HalonHSLRegisterContext
) -> bool {
    let func_name = std::ffi::CString::new("rust_example").unwrap();
    unsafe {
        HalonMTA_hsl_module_register_function(hhrc, func_name.as_ptr(), Some(rust_example));
    }
    return true
}
