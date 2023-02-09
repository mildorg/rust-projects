use super::{insert_sort, utils::pos_comparator};

fn bucket_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    let bucket_size = 5;
    let (max, min) = get_max_min(list);
    let space = max - min;

    let bucket_count = (space / bucket_size + 1) as usize;
    let mut buckets = create_buckets(bucket_count);

    // list 元素入桶
    for n in list.iter() {
        let index = ((n - min) / bucket_size) as usize;
        buckets[index].push(*n);
    }

    // 排序
    let mut index = 0;

    for bucket in buckets.iter_mut() {
        insert_sort::insert_sort(bucket); // 使用插入排序，排序桶中的元素

        for n in bucket.iter() {
            list[index] = *n;
            index += 1;
        }
    }
}

fn get_max_min(list: &[i32]) -> (i32, i32) {
    let mut max = list[0];
    let mut min = list[0];

    (1..list.len()).for_each(|i| {
        if list[i] > max {
            max = list[i];
        }

        if list[i] < min {
            min = list[i];
        }
    });

    (max, min)
}

fn create_buckets(count: usize) -> Vec<Vec<i32>> {
    let mut buckets = Vec::with_capacity(count);

    for _ in 0..count {
        buckets.push(vec![]);
    }

    buckets
}

pub fn test() {
    let passed = pos_comparator(bucket_sort, 10_000);
    println!("The bucket sort works well? {passed}");
}
