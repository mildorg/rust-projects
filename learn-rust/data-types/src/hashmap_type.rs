use std::collections::HashMap;

pub fn learn_hashmap() {
    // _create_hashmap();
    // _vector_to_map();
    // _ownership_map();
    // _get_elements();
    // _update_map();
    _count_word();
}


pub fn _create_hashmap() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
}

pub fn _vector_to_map() {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<String, u32> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);
}

pub fn _ownership_map() {
    let filed_name = String::from("Favorite color");
    let filed_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(filed_name, filed_value);

    println!("{:?}", &map);
    // println!("{}", filed_name);
    // println!("{}", filed_value);
}

pub fn _get_elements() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("The value is {}", s),
        None => println!("The value is none"),
    }

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

pub fn _update_map() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}

pub fn _count_word() {
    let text = "Hello word wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
