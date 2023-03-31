use std::cmp::Ordering;

use super::utils::comparator;

fn quick_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    sort(list, 0, len - 1);
}

fn sort(list: &mut [i32], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let (less, more) = partition(list, l, r);

    if less > 0 {
        sort(list, 0, less - 1);
    }

    sort(list, more + 1, r);
}

fn partition(list: &mut [i32], l: usize, r: usize) -> (usize, usize) {
    let mut less = l;
    let mut more = r;

    let mut index = l;
    let base = list[r];

    while index <= more {
        match list[index].cmp(&base) {
            Ordering::Less => {
                list.swap(less, index);
                less += 1;
                index += 1;
            }
            Ordering::Equal => {
                index += 1;
            }
            Ordering::Greater => {
                list.swap(more, index);
                more -= 1;
            }
        }
    }

    (less, more)
}

pub fn test() {
    let passed = comparator(quick_sort, 1_000);
    println!("The quick sort is works well: {passed} !");
}
