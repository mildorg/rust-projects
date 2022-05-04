use std::fmt::Display;

pub fn learn() {
    // drop_point();
    // fn_lifetime();
    struct_lifetime();
}

fn drop_point() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }

    let x = 5;
    let r = &x;

    println!("r:{}", r);
}

fn fn_lifetime() {
    let string1 = String::from("long string is long");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // {
    //     let string2 = String::from("xyz");
    //     let result = longest(string1.as_str(), string2.as_str());
    //     println!("The longest string is {}", result);
    // }

    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
//     x
// }

fn longest(_x: &str, _y: &str) -> String {
    String::from("really long string")
}

fn struct_lifetime() {}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", &ann);

    if x.len() > y.len() {
        x
    } else {
        y
    }
}
