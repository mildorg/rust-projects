use rand::Rng;

pub fn comparator<F>(sort_fn: F, times: u32) -> bool
where
    F: Fn(&mut Vec<i32>),
{
    for _ in 0..times {
        let mut list1 = get_rand_list();
        let mut list2 = list1.clone();

        sort_fn(&mut list1);
        list2.sort();

        if !is_equal(&list1, &list2) {
            println!("Custom sort list1: {list1:?}");
            println!("Std sort list2: {list2:?}");
            return false;
        }
    }

    true
}

fn is_equal(list1: &Vec<i32>, list2: &Vec<i32>) -> bool {
    if list1.len() != list2.len() {
        return false;
    }

    for i in 0..list1.len() {
        if list1[i] != list2[i] {
            return false;
        }
    }

    true
}

fn get_rand_list() -> Vec<i32> {
    let len = rand::thread_rng().gen_range(0..100);
    let mut list = Vec::with_capacity(100);

    for _ in 0..len {
        let v1 = rand::thread_rng().gen_range(0..100);
        let v2 = rand::thread_rng().gen_range(0..100);

        list.push(v1 - v2);
    }

    list
}
