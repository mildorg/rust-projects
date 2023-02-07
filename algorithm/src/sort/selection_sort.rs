use super::utils::comparator;

// 选择排序
fn selection_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    for i in 0..len {
        let mut min = i;

        for j in i + 1..len {
            if list[j] < list[min] {
                min = j;
            }
        }

        if min != i {
            list.swap(min, i);
        }
    }
}

pub fn test() {
    let passed = comparator(selection_sort, 10_000);
    println!("The selection_sor is work well?: {passed}");
}
