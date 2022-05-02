pub fn learn_vec() {
  // let v: Vec<i32> = Vec::new();
  // let mut v = vec![1, 2, 3];
  // v.push(4);
  // v.push(5);
  // println!("The value of v is {:?}", v);
  // get_element();
  // _read_and_push();
  // _read_all_elements();
  // _change_elements();
  // _enum_vector();
}

pub fn _get_element() {
  let v = vec![1, 2, 3, 4, 5];
  let third = &v[2];
  println!("The third element is {}", third);

  match v.get(100) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element"),
  }
}

pub fn _read_and_push() {
  let mut v = vec![1, 2, 3, 4, 5];
  let _third = &v[0];
  v.push(7);
}

pub fn _read_all_elements() {
  let v = vec![100, 32, 57];
  for i in v {
    println!("{}", i);
  }
}

pub fn _change_elements() {
  let mut v = vec![100, 32, 57];
  for i in &mut v {
    *i += 50;
  }

  for i in &v {
    println!("{}", i);
  }
}

enum SpreadsheetCell {
  Int(i32),
  Float(f64),
  Text(String),
}

pub fn _enum_vector() {
  let _row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
  ];
}
