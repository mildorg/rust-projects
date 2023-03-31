use super::utils::comparator;

// 选择排序
fn selection_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    for i in 0..len {
        let mut min = i;

        for p in i + 1..len {
            if list[p] < list[min] {
                min = p;
            }
        }

        if min != i {
            list.swap(i, min);
        }
    }
}

pub fn test() {
    let passed = comparator(selection_sort, 1_000);
    println!("The selection_sor is work well?: {passed}");
}
