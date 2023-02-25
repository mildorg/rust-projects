#![allow(dead_code)]

use std::cell::UnsafeCell;

pub fn test_miri() {
    // test1();
    // test2();
    // test_arr();
    // test_arr2();
    // test_arr3();
    // test_arr4();
    // test_arr5();
    // test_borrow1();
    test_borrow2();
}

fn test1() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        *ptr2 += 2;

        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{data}");
    }
}

fn test2() {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        // Access things in "borrow stack" order
        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{data}");
    }
}

fn test_arr() {
    unsafe {
        let mut data = [0; 10];
        let ref1_at_0 = &mut data[0];
        let ptr2_at_0 = ref1_at_0 as *mut i32;
        let ptr3_at_1 = ptr2_at_0;
        // let ptr3_at_1 = ptr2_at_0.add(1);

        *ptr3_at_1 += 3;
        *ptr2_at_0 += 2;
        *ref1_at_0 += 1;

        println!("{:?}", &data)
    }
}

fn test_arr2() {
    unsafe {
        let mut data = [0; 10];
        let ref1_0 = &mut data[0];
        let pt2_0 = ref1_0 as *mut i32;
        let pt3_0 = pt2_0;
        let ptr4_0 = pt3_0.add(0);
        let ptr5_0 = ptr4_0.add(1).sub(1);

        *pt3_0 += 3;
        *pt2_0 += 2;
        *ptr4_0 += 4;
        *ptr5_0 += 5;
        *pt3_0 += 3;
        *pt2_0 += 2;
        *ref1_0 += 1;

        println!("{:?}", &data);
    }
}

fn test_arr3() {
    // unsafe {
    //     let mut data = [0; 10];
    //     let ref1_0 = &mut data[0];
    //     let ref2_1 = &mut data[1];
    //     let pt3_0 = ref1_0 as *mut i32;
    //     let pt4_1 = ref2_1 as *mut i32;

    //     *pt4_1 += 4;
    //     *pt3_0 += 3;
    //     *ref2_1 += 2;
    //     *ref1_0 += 1;

    //     println!("{:?}", &data)
    // }
}

fn test_arr4() {
    unsafe {
        let mut data = [0; 10];
        let slice1 = &mut data[..];
        let (slice2_0, slice3_1) = slice1.split_at_mut(1);
        let ref4_0 = &mut slice2_0[0];
        let ref5_1 = &mut slice3_1[0];
        let ptr6_0 = ref4_0 as *mut i32;
        let ptr7_1 = ref5_1 as *mut i32;

        *ptr7_1 += 7;
        *ptr6_0 += 6;
        *ref5_1 += 5;
        *ref4_0 += 4;

        println!("{:?}", &data);
    }
}

fn test_arr5() {
    unsafe {
        let mut data = [0; 10];
        let slice1_all = &mut data[..];
        let ptr2_all = slice1_all.as_mut_ptr();

        let ptr3_0 = ptr2_all;
        let ptr4_1 = ptr2_all.add(1);
        let ref5_0 = &mut *ptr3_0;
        let ref6_1 = &mut *ptr4_1;

        *ref6_1 += 6;
        *ref5_0 += 5;
        *ptr4_1 += 4;
        *ptr3_0 += 3;

        for i in 0..10 {
            *ptr2_all.add(i) += i;
        }

        for (i, elem) in slice1_all.iter_mut().enumerate() {
            *elem += i;
        }

        println!("{:?}", &data);
    }
}

fn opaque_read(val: &i32) {
    println!("{val}");
}

fn test_borrow1() {
    let mut data = 10;
    let mref1 = &mut data;
    let sref2 = &mref1;
    let sref3 = sref2;

    #[allow(clippy::borrow_deref_ref)]
    let sref4 = &(*sref2);

    opaque_read(sref3);
    opaque_read(sref2);
    opaque_read(sref4);
    opaque_read(sref2);
    opaque_read(sref3);

    *mref1 += 1;
    opaque_read(&data);
}

fn test_borrow2() {
    unsafe {
        let mut data = UnsafeCell::new(10);
        let mref1 = &mut data; // Mutable ref to the *outside*
        let ptr2 = mref1.get(); // Get a raw pointer to the insides
        let sref3 = &*mref1; // Get a shared ref to the *outside*

        *ptr2 += 2; // Mutate with the raw pointer
        opaque_read(&*sref3.get()); // Read from the shared ref
        *sref3.get() += 3; // Write through the shared ref
        *mref1.get() += 1; // Mutate with the mutable ref

        println!("{}", *data.get());
    }
}
