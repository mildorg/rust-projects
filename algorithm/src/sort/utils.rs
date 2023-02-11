use std::time::Instant;

use rand::Rng;

pub fn comparator<F: Fn(&mut Vec<i32>)>(sort_fn: F, times: u32) -> bool {
    compare(sort_fn, false, times)
}

pub fn pos_comparator<F: Fn(&mut Vec<i32>)>(sort_fn: F, times: u32) -> bool {
    compare(sort_fn, true, times)
}

fn compare<F>(sort_fn: F, positive: bool, times: u32) -> bool
where
    F: Fn(&mut Vec<i32>),
{
    let start = Instant::now();

    for _ in 0..times {
        let mut list1 = get_rand_list(positive);
        let mut list2 = list1.clone();

        sort_fn(&mut list1);
        list2.sort();

        if !is_equal(&list1, &list2) {
            println!("Custom sort list1: {list1:?}");
            println!("Std sort list2: {list2:?}");
            return false;
        }
    }

    let duration = start.elapsed();
    println!();
    println!("The expensive time is {duration:?}");

    true
}

pub fn is_equal(list1: &Vec<i32>, list2: &Vec<i32>) -> bool {
    if list1.len() != list2.len() {
        return false;
    }

    for i in 0..list1.len() {
        if list1[i] != list2[i] {
            return false;
        }
    }

    true
}

fn get_rand_list(positive: bool) -> Vec<i32> {
    let size = 100;
    let len = rand::thread_rng().gen_range(0..size);
    let mut list = Vec::with_capacity(1000);

    for _ in 0..len {
        let mut value = rand::thread_rng().gen_range(0..size);

        if !positive {
            value -= rand::thread_rng().gen_range(0..size)
        }

        list.push(value);
    }

    list
}
