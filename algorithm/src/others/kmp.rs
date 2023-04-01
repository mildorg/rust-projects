pub fn process(str1: &str, str2: &str) -> i32 {
    let str1_len = str1.len();
    let str2_len = str2.len();

    if str2_len == 0 || str2_len > str1_len {
        return -1;
    }

    let next = get_next_arr(str2);
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < str1_len && i2 < str2_len {
        if get_str(str1, i1) == get_str(str2, i2) {
            i1 += 1;
            i2 += 1;
        } else if next[i2] == -1 {
            i1 += 1;
        } else {
            i2 = next[i2] as usize;
        }
    }

    match i2 == str2_len {
        true => (i1 - i2) as i32, // 如果i2越界
        false => -1,
    }
}

fn get_next_arr(str: &str) -> Vec<i32> {
    let len = str.len();
    let mut next = vec![0; len];
    next[0] = -1;

    let mut i = 2;
    let mut cn = 0;

    while i < len {
        if get_str(str, i - 1) == get_str(str, cn as usize) {
            cn += 1;
            next[i] = cn;
            i += 1;
        } else if cn == 0 {
            i += 1;
        } else {
            cn = next[cn as usize];
        }
    }

    next
}

fn get_str(str: &str, i: usize) -> &str {
    &str[i..i + 1]
}

#[cfg(test)]
mod test {
    use super::process;

    // noinspection All
    #[test]
    fn basics() {
        let str1 = "12abcedfabcedf00";
        let str2 = "abcedfabcedf";
        let index = process(str1, str2);
        assert_eq!(index, 2);

        let str1 = "12abcedfabcedf00";
        let str2 = "abcedfabcedfe";
        let index = process(str1, str2);
        assert_eq!(index, -1);

        let str1 = "abcedfabcedf00";
        let str2 = "abcedfabcedf";
        let index = process(str1, str2);
        assert_eq!(index, 0);
    }
}
