pub fn process() {
    let mut are = [
        vec![1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0, 1, 1, 1, 0],
        vec![0, 1, 1, 1, 0, 0, 0, 1, 0],
        vec![0, 1, 1, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0],
    ];

    // 遍历区域，寻找感染之后 are[i][j] = 1的数量
    let mut count = 0;
    let n = are.len();
    let m = are[0].len();

    for i in 0..n {
        (0..m).for_each(|j| {
            if are[i][j] == 1 {
                count += 1;
                infect(&mut are, i, j, n, m);
            }
        })
    }

    println!("The lands count is: {count}");
}

/// * are: 区域，是一个矩阵
/// * i: 元素在矩阵中的行位置索引
/// * j: 元素在矩阵中的列位置索引
/// * n: 矩阵行的长度
/// * m: 矩阵列的长度
fn infect(are: &mut [Vec<i32>], i: usize, j: usize, n: usize, m: usize) {
    if i >= n || j >= m || are[i][j] != 1 {
        return;
    }

    // 将当前位置设置为2 标识为已经找到的位置
    are[i][j] = 2;
    // 从当前位置向上下左右寻找
    infect(are, i + 1, j, n, m);
    infect(are, i, j + 1, n, m);

    if i > 0 {
        infect(are, i - 1, j, n, m);
    }

    if j > 0 {
        infect(are, i, j - 1, n, m);
    }
}
