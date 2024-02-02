use lean_sys::{lean_array_push, lean_mk_empty_array, lean_mk_string, lean_obj_res};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn arr() -> lean_obj_res {
    let x = unsafe { lean_mk_empty_array() };
    // unsafe { lean_array_push(x, lean_mk_string("NamedNode".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("http://example.org/test".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("NamedNode".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("http://example.org/predicate".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("Literal".as_ptr())) };
    // unsafe { lean_array_push(x, lean_mk_string("belting".as_ptr())) };
    return x;
}

#[cfg(test)]
mod tests {
    use super::*;
    use lean_sys::lean_array_size;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn arr_works() {
        let result = arr();
        assert_eq!(unsafe { lean_array_size(lean_mk_empty_array()) }, 6);
    }
}
