pub fn _learn_string() {
  // _concat_str();
  _loop_str()
}


pub fn add (){
  let s1 = String::from();
}

pub fn _add_str() {
  let mut str1 = String::from("foo");
  let str2 = "bar";

  str1.push_str(str2);
  println!("{}", str1);
}

pub fn _concat_str() {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  // let s = s1 + "-" + &s2 + "-" + &s3;
  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("{}", s);
}

// pub fn _index_str() {
//   let s = String::from("hello");
//   let h = s[0];
// }

pub fn _loop_str() {
  // for c in "नमस्ते".chars() {
  //   println!("{}", c);
  // }

  for b in "नमस्ते".bytes() {
    println!("{}", b);
  }
}
