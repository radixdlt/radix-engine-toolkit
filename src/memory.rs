/// Allocates memory of a specific capacity and returns a pointer to this memory back to the caller.
#[no_mangle]
pub extern "C" fn __transaction_lib_alloc(capacity: usize) -> *mut std::os::raw::c_char {
    let mut buf = Vec::with_capacity(capacity);
    let ptr = buf.as_mut_ptr();

    std::mem::forget(buf);

    ptr
}

/// Frees up the memory used up at a given pointer.
/// 
/// # Safety
/// 
/// This function makes use of pointers which is an unsafe feature.
#[no_mangle]
pub unsafe extern "C" fn __transaction_lib_free(pointer: *mut std::os::raw::c_char) {
    if !pointer.is_null() {
        drop(std::ffi::CString::from_raw(pointer));
    }
}
