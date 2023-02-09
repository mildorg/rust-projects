use super::utils::comparator;

fn heap_sort(list: &mut Vec<i32>) {
    let len = list.len();

    if len <= 1 {
        return;
    }

    // 从最右子树开始堆化数组
    for j in (0..len).rev() {
        heapify(list, j, len);
    }

    // 排序数组
    let mut j = len - 1;
    while j > 0 {
        list.swap(0, j);
        heapify(list, 0, j);
        j -= 1;
    }
}

fn heapify(list: &mut [i32], head: usize, size: usize) {
    let mut i = head; // 父节点
    let mut left = 2 * i + 1;

    // 有子节点
    while left < size {
        // 最大子节点的index
        let right = left + 1;
        let largest = if right < size && list[right] > list[left] {
            right
        } else {
            left
        };

        // 父节点更大
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
    println!("The heap sort works well: {passed}");
}
