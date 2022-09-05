pub fn learn() {
    let s = Single(A);
    let char = SingleGen('a');
    let t = SingleGen(A);
    let _i32 = SingleGen(6);
}

struct A;

struct Single(A);

struct SingleGen<T>(T);
