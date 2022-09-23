use std::slice;

pub(crate) fn learn() {
    // raw_pointers();
    unsafe_functions();
}

fn raw_pointers() {
    let raw_p: *const u32 = &10;

    // assert!(*raw_p == 10);

    unsafe {
        assert!(*raw_p == 10);
    }
}

fn unsafe_functions() {
    let vector = vec![1, 2, 3, 4];

    let pointer = vector.as_ptr();
    let len = vector.len();

    unsafe {
        let my_slice = slice::from_raw_parts(pointer, len);
        assert_eq!(vector.as_slice(), my_slice);
    }
}
