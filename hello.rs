#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "C" fn Halon_version(
) -> u32 {
    HALONMTA_PLUGIN_VERSION
}

pub extern "C" fn hello(
    _hhc: *mut HalonHSLContext,
    _args: *mut HalonHSLArguments,
    ret: *mut HalonHSLValue,
) {
    let world = std::ffi::CString::new("world").unwrap();
    unsafe {
        HalonMTA_hsl_value_set(ret, HALONMTA_HSL_TYPE_STRING as i32, world.as_ptr() as *mut std::ffi::c_void, 0);
    }
}

#[no_mangle]
pub extern "C" fn Halon_hsl_register(hhrc: *mut HalonHSLRegisterContext
) -> bool {
    let func_name = std::ffi::CString::new("hello").unwrap();
    unsafe {
        HalonMTA_hsl_module_register_function(hhrc, func_name.as_ptr(), Some(hello));
    }
    return true
}
