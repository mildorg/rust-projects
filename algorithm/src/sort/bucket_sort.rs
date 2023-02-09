use super::{insert_sort, utils::pos_comparator};

fn bucket_sort(list: &mut Vec<i32>) {
    if list.len() <= 1 {
        return;
    }

    let size = 5;
    let (max, min) = get_max_min(list);
    let count = ((max - min) / size + 1) as usize;
    let mut buckets = Vec::with_capacity(count);

    // 创建桶
    for _ in 0..count {
        buckets.push(vec![]);
    }

    // 数据入桶
    (0..list.len()).for_each(|i| {
        let index = ((list[i] - min) / size) as usize;
        buckets[index].push(list[i]);
    });

    // 排序
    let mut index = 0;
    for bucket in buckets.iter_mut() {
        insert_sort::insert_sort(bucket);

        for n in bucket {
            list[index] = *n;
            index += 1;
        }
    }
}

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
    let passed = pos_comparator(bucket_sort, 10_000);
    println!("The bucket sort 2 works well? {passed}");
}
