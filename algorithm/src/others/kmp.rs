pub fn process(str1: &str, str2: &str) -> i32 {
    let st1_len = str1.len();
    let str2_len = str2.len();

    if str2_len == 0 || str2_len > st1_len {
        return -1;
    }

    let next = get_next_arr(str2);
    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < st1_len && i2 < str2_len {
        if get_str(str1, i1) == get_str(str2, i2) {
            i1 += 1;
            i2 += 1;
        } else if next[i2] == -1 {
            i1 += 1;
        } else {
            i2 = next[i2] as usize;
        }
    }

    // 如果 i2 先越界
    match i2 == str2_len {
        true => (i1 - i2).try_into().unwrap(),
        false => -1,
    }
}



fn get_next_arr(s: &str) -> Vec<i32> {
    let len = s.len();

    let mut next = vec![-1; len];
    next[1]=0;

    let mut i = 2;
    let mut cn = 0;

    while i < len {
        if get_str(s,i-1) == get_str(s, cn as usize) {
            cn += 1;
            next[i] = cn;
            i += 1;
        } else if cn == 0 {
            i += 1;
        } else {
            cn = next[cn as usize]
        }
    }

    next
}

fn get_str(str: &str, i: usize) -> &str {
    &str[i..i+1]
}


