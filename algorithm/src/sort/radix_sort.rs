use std::collections::VecDeque;

use super::utils::pos_comparator;

/// 基数排序
fn radix_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    //创建桶
    let mut buckets = Vec::with_capacity(10);
    for _ in 0..10 {
        buckets.push(VecDeque::new());
    }

    // 排序
    let mut n = get_max(list);
    let mut d = 1;

    while n > 0 {
        // 输入入桶
        (0..len).for_each(|i| {
            let index = (list[i] / d) as usize % 10;
            buckets[index].push_back(list[i]);
        });

        // 数据入 list
        let mut i = 0;
        for bucket in buckets.iter_mut() {
            while !bucket.is_empty() {
                list[i] = bucket.pop_front().unwrap();
                i += 1;
            }
        }

        // 对下一进制位进行处理
        n /= 10;
        d *= 10;
    }
}

fn get_max(list: &[i32]) -> i32 {
    let mut max = list[0];

    (1..list.len()).for_each(|i| {
        if list[i] > max {
            max = list[i];
        }
    });

    max
}

pub fn test() {
    let passed = pos_comparator(radix_sort, 1_000);
    println!("The radix sort 2 works well: {passed}");
}
