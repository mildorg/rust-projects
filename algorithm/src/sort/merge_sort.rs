use super::utils::comparator;

fn merge_sort(list: &mut Vec<i32>) {
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

    let m = l + ((r - l) >> 1);
    sort(list, l, m);
    sort(list, m + 1, r);
    merge(list, l, m, r);
}

fn merge(list: &mut [i32], l: usize, m: usize, r: usize) {
    let mut help = Vec::with_capacity(r - l + 1);
    let mut p1 = l;
    let mut p2 = m + 1;

    while p1 <= m && p2 <= r {
        if list[p1] <= list[p2] {
            help.push(list[p1]);
            p1 += 1;
        } else {
            help.push(list[p2]);
            p2 += 1;
        }
    }

    // 左边有剩余
    while p1 <= m {
        help.push(list[p1]);
        p1 += 1;
    }

    // 右边有剩余
    while p2 <= r {
        help.push(list[p2]);
        p2 += 1;
    }

    // 将help 中的元素copy 到list中
    (0..help.len()).for_each(|i| {
        list[l + i] = help[i];
    });
}

pub fn test() {
    let passed = comparator(merge_sort, 10_000);
    println!("The merge sort works well: {passed}");
}
