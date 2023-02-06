use std::cmp::Ordering;

use super::utils::comparator;

fn quick_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    process(list, 0, len - 1);
}

fn process(list: &mut [i32], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let (low, hight) = partition(list, l, r);

    if low > 1 {
        process(list, 0, low - 1);
    }

    if hight + 1 < r {
        process(list, hight + 1, r);
    }
}

fn partition(list: &mut [i32], l: usize, r: usize) -> (usize, usize) {
    if l > r {
        return (0, r);
    }

    if l == r {
        return (l, r);
    }

    let mut less = if l == 0 { -1 } else { (l - 1) as i32 };
    let mut more = r;

    let mut index = l;
    let base = list[r];

    while index < more {
        match list[index].cmp(&base) {
            Ordering::Less => {
                list.swap((less + 1) as usize, index);
                less += 1;
                index += 1;
            }
            Ordering::Equal => {
                index += 1;
            }
            Ordering::Greater => {
                list.swap(more - 1, index);
                more -= 1;
            }
        }
    }

    list.swap(more, r);
    ((less + 1) as usize, more)
}

pub fn test() {
    let passed = comparator(quick_sort, 100_000);
    println!("The quick sort is works well: {passed} !");
}
