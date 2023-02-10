use std::collections::VecDeque;

use super::utils::pos_comparator;

/// 基数排序
fn radix_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    // 创建桶
    let mut buckets = Vec::with_capacity(10);
    for _ in 0..10 {
        buckets.push(VecDeque::new());
    }

    // 排序
    let mut n = get_max(list);
    let mut d = 1;

    while n > 0 {
        // 数据入桶
        (0..len).for_each(|i| {
            let index = (list[i] / d) as usize % 10;
            buckets[index].push_back(list[i]);
        });

        // 将数据重新 copy 到 list
        let mut i = 0;
        for bucket in buckets.iter_mut() {
            while !bucket.is_empty() {
                list[i] = bucket.pop_front().unwrap();
                i += 1;
            }
        }

        // 对下一个10进位进行操作
        n /= 10;
        d *= 10;
    }
}

fn get_max(list: &[i32]) -> i32 {
    let mut max = list[0];

    for n in list.iter().skip(1) {
        let n = *n;

        if n > max {
            max = n;
        }
    }

    max
}

pub fn test() {
    let passed = pos_comparator(radix_sort, 1_000);
    println!("The radix sort 2 works well: {passed}");
}
