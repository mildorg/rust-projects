use super::utils::comparator;

pub fn insert_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    for i in 1..len {
        let mut j = i;

        while j > 0 && list[j] < list[j - 1] {
            list.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn test() {
    let passed = comparator(insert_sort, 1_000);
    println!("The insert sort work well?: {passed}");
}
