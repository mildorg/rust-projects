use super::utils::comparator;

// 选择排序
fn selection_sor(list: &mut Vec<i32>) {
    let len = list.len();
    if len == 0 || len == 1 {
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
            list.swap(i, min);
        }
    }
}

pub fn test() {
    let passed = comparator(selection_sor, 10_000);
    println!("The selection_sor is work well?: {passed}");
}
