use lean_sys::lean_mk_string_from_bytes;

#[no_mangle]
pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {
    let _x = unsafe { lean_mk_string_from_bytes("1".as_ptr(), 1) };
    return a + b
}
