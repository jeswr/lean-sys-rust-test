use std::slice;

use lean_sys::{lean_array_push, lean_array_size, lean_array_uget, lean_mk_empty_array, lean_mk_string_from_bytes, lean_obj_res, lean_string_cstr, lean_string_len};

pub fn lean_mk_rust_string(s: &str) -> lean_obj_res {
    unsafe { lean_mk_string_from_bytes(s.as_ptr(), s.len()) }
}

pub fn lean_to_rust_string(s: lean_obj_res) -> &'static str {
    let ptr = unsafe { lean_string_cstr(s) };
    let len = unsafe { lean_string_len(s) };

    // We can re-build a str out of ptr and len. This is all unsafe because
    // we are responsible for making sure the two components are valid:
    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(ptr, len);

        // ... and then convert that slice into a string slice
        std::str::from_utf8(slice)
    };

    return s.unwrap();
}

pub fn lean_mk_string_array(strings: Vec<&str>) -> lean_obj_res {
    let mut x = unsafe { lean_mk_empty_array() };
    for s in strings {
        x = unsafe { lean_array_push(x, lean_mk_rust_string(s)) };
    }
    return x;
}

pub fn array_from_lean_string_array(lean_array: lean_obj_res) -> Vec<&'static str> {
    let mut result = Vec::new();
    let size = unsafe { lean_array_size(lean_array) };
    for i in 0..size {
        let item = lean_to_rust_string(unsafe { lean_array_uget(lean_array, i) });
        result.push(item);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_sys::{lean_array_size, lean_initialize};

    #[test]
    fn arr_works() {
        unsafe { lean_initialize() };
        assert_eq!(lean_to_rust_string(lean_mk_rust_string("Hello")), "Hello");

        let vec: Vec<&str> = ["NamedNode", "http://example.org/test", "NamedNode", "http://example.org/predicate", "Literal", "belting"].to_vec();

        let result = lean_mk_string_array(vec.clone());
        assert_eq!(unsafe { lean_array_size(result) }, 6);
        assert_eq!(array_from_lean_string_array(result), vec);
        assert_eq!(unsafe { lean_array_size(result) }, 6);
    }
}
