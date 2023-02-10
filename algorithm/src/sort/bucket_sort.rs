use super::{insert_sort, utils::pos_comparator};

fn bucket_sort(list: &mut Vec<i32>) {
    if list.len() <= 1 {
        return;
    }

    let size = 5;
    let (max, min) = get_max_min(list);
    let count = ((max - min) / size + 1) as usize;
    let mut buckets: Vec<Vec<i32>> = Vec::with_capacity(count);

    // 创建桶
    for _ in 0..count {
        buckets.push(vec![]);
    }

    // 数据入桶
    for n in list.iter() {
        let i = ((n - min) / size) as usize;
        buckets[i].push(*n);
    }

    // 排序
    let mut i = 0;
    for bucket in buckets.iter_mut() {
        insert_sort::insert_sort(bucket);

        for n in bucket {
            list[i] = *n;
            i += 1;
        }
    }
}

// 获取最大和最小
fn get_max_min(list: &[i32]) -> (i32, i32) {
    let mut max = list[0];
    let mut min = list[0];

    for n in list.iter().skip(1) {
        let n = *n;

        if n > max {
            max = n;
        }

        if n < min {
            min = n;
        }
    }

    (max, min)
}

pub fn test() {
    let passed = pos_comparator(bucket_sort, 1_000);
    println!("The bucket sort works well? {passed}");
}
