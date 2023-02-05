use super::utils::comparator;

// 选择排序
fn selection_sor(list: &mut Vec<i32>) {
    let len = list.len();
    let mut min;

    // 0 ~ n-1  找到最小值，在哪，放到0位置上
    // 1 ~ n-1  找到最小值，在哪，放到1 位置上
    // 2 ~ n-1  找到最小值，在哪，放到2 位置上
    for i in 0..len {
        min = i;

        for j in i + 1..len {
            if list[j] < list[min] {
                min = j;
            }
        }

        if min != i {
            list.swap(i, min)
        }
    }
}

pub fn test() {
    let passed = comparator(selection_sor, 100_000);
    println!("selection_sor is work well?: {passed}");
}
