mod front_of_house;

pub use self::front_of_house::hosting;
use std::collections::HashMap;
// use std::fmt::Result;
// use std::io::Result as IoResult;
use std::io::{self, Write};
// use std::{cmp::Ordering, io};

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_wait_list();

    // 相对路径
    front_of_house::hosting::add_to_wait_list();

    let mut meal = front_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    let order1 = front_of_house::Appetize::Soup;
    let order2 = front_of_house::Appetize::Salad;

    hosting::add_to_wait_list();
    let mut map: HashMap<u32, u32> = HashMap::new();
    map.insert(1, 2);
}
