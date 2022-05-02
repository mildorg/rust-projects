fn main() {
    // 数据竞争
    let mut s = String::from("Hello");

    let s1 = &s;
    let s2 = &s;
    let s3 = &mut s;

    println!("{} , {}", s1, s2);
}
