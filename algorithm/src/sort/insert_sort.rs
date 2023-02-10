use super::utils::comparator;

pub fn insert_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    for i in 0..len {
        let mut index = i;

        while index > 0 && list[index] < list[index - 1] {
            list.swap(index, index - 1);
            index -= 1;
        }
    }
}

pub fn test() {
    let passed = comparator(insert_sort, 1_000);
    println!("The insert sort work well?: {passed}");
}
