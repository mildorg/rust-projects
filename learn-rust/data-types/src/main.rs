// mod string_type;
// mod vector_type;
mod hashmap_type;

fn main() {
    // let x = 2.0;
    // let x: f32 = 6.4;

    // 语句和表达式
    // let x = (let y =6)

    // let y = {
    //     let x = 3;
    //     x + 2
    // };

    // let y = five();
    //
    // println!("The value of y is {}", y);

    // let x = plus_one(5);
    // println!("The value of x is {}", x);

    // control_flow();
    // loop_flow();
    // while_flow();
    // for_flow();

    // vector
    // vector_type::learn_vec();
    // string_type::_learn_string();
    hashmap_type::learn_hashmap();
}

// fn five() -> u32 {
//     return 5;
// }

// fn plus_one(x: u32) -> u32 {
//     x + 2
// }

// fn control_flow() {
//     // if
//     let n = 6;

//     // if n < 5 {
//     //     println!("condition was true");
//     // } else {
//     //     println!("condition was false");
//     // }

//     // if n != 0 {
//     //     println!("Number was something other than zero!");
//     // }

//     // if n % 4 == 0 {
//     //     println!("Number is divisible by 4.");
//     // } else if n % 3 == 0 {
//     //     println!("Number is divisible by 3.");
//     // } else if n % 2 == 0 {
//     //     println!("Number is divisible by 2.");
//     // } else {
//     //     println!("Number is not divisible by 4、3 or 2.");
//     // }

//     let y = if n < 6 { true } else { false };
//     println!("The value of y is {}", y)
// }

// fn loop_flow() {
//     // loop {
//     //     println!("again！");
//     // }

//     // let mut count = 0;

//     // 'counting_up: loop {
//     //     println!("count = {}", count);

//     //     let mut remaining = 10;

//     //     loop {
//     //         println!("remaining = {}", remaining);

//     //         if remaining == 9 {
//     //             break;
//     //         }

//     //         if count == 2 {
//     //             break 'counting_up;
//     //         }

//     //         remaining -= 1;
//     //     }

//     //     count += 1;
//     // }

//     // println!("End count = {}", count);

//     let mut count = 0;

//     let result = loop {
//         count += 1;

//         if count == 10 {
//             break count * 2;
//         }
//     };

//     println!("The value of result is {}", result);
// }

// fn while_flow() {
//     let mut number = 3;

//     while number != 0 {
//         println!("The number is {}", number);
//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// fn for_flow() {
//     // let arr = [5, 4, 3, 2, 1];

//     // for elem in &arr {
//     //     println!("The value is : {}", elem);
//     // }

//     for number in 1..4 {
//         println!("The value is: {}", number);
//     }
// }
