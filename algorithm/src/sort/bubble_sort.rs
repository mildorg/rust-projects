use super::utils::comparator;

fn bubble_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    for i in 0..len {
        for j in 0..len - i - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
}

pub fn test() {
    let passed = comparator(bubble_sort, 10_000);
    println!("The bubble_sort works well? {passed}");
}
