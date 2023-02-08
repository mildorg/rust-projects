use super::utils::comparator;

fn heap_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    // 从最右子树堆化列表
    for i in (0..len).rev() {
        heapify(list, i, len);
    }

    // 排序
    let mut j = len - 1;
    while j > 0 {
        list.swap(0, j);
        heapify(list, 0, j);
        j -= 1;
    }
}

fn heapify(list: &mut [i32], last_head: usize, size: usize) {
    let mut i = last_head;
    let mut left = 2 * i + 1;

    // 如果存在子节点
    while left < size {
        // 最大的子节点index
        let largest = if left + 1 < size && list[left + 1] > list[left] {
            left + 1
        } else {
            left
        };

        // 如果父节点更大
        if list[i] >= list[largest] {
            break;
        }

        list.swap(i, largest);
        i = largest;
        left = 2 * i + 1;
    }
}

pub fn test() {
    let passed = comparator(heap_sort, 10_000);
    println!("The heap sort 33 works well: {passed}");
}
