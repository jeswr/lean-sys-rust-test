#[cfg(test)]
mod tests {
    use lean_sys::lean_mk_empty_array;

    #[test]
    fn arr_works() {
        let _a = unsafe { lean_mk_empty_array() };
    }
}
