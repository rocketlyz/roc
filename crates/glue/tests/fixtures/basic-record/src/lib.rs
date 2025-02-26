use roc_app;

#[no_mangle]
pub extern "C" fn rust_main() -> i32 {
    use std::cmp::Ordering;
    use std::collections::hash_set::HashSet;

    let record = roc_app::mainForHost();

    // Verify that the record has all the expected traits.

    assert!(record == record); // PartialEq
    assert!(record.clone() == record.clone()); // Clone

    // Since this is a move, later uses of `record` will fail unless `record` has Copy
    let rec2 = record; // Copy

    assert!(rec2 != Default::default()); // Default
    assert!(record.partial_cmp(&record) == Some(Ordering::Equal)); // PartialOrd
    assert!(record.cmp(&record) == Ordering::Equal); // Ord

    let mut set = HashSet::new();

    set.insert(record); // Eq, Hash
    set.insert(rec2);

    assert_eq!(set.len(), 1);

    println!("Record was: {:?}", record); // Debug

    // Exit code
    0
}

// Externs required by roc_std and by the Roc app

use core::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn roc_alloc(size: usize, _alignment: u32) -> *mut c_void {
    return libc::malloc(size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_realloc(
    c_ptr: *mut c_void,
    new_size: usize,
    _old_size: usize,
    _alignment: u32,
) -> *mut c_void {
    return libc::realloc(c_ptr, new_size);
}

#[no_mangle]
pub unsafe extern "C" fn roc_dealloc(c_ptr: *mut c_void, _alignment: u32) {
    return libc::free(c_ptr);
}

#[no_mangle]
pub unsafe extern "C" fn roc_panic(c_ptr: *mut c_void, tag_id: u32) {
    match tag_id {
        0 => {
            let slice = CStr::from_ptr(c_ptr as *const c_char);
            let string = slice.to_str().unwrap();
            eprintln!("Roc hit a panic: {}", string);
            std::process::exit(1);
        }
        _ => todo!(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn roc_memset(dst: *mut c_void, c: i32, n: usize) -> *mut c_void {
    libc::memset(dst, c, n)
}
