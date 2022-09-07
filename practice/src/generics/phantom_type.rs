use std::marker::PhantomData;

pub fn learn() {
    let tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let tuple2: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    // let tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    let struct1: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    let struct2: PhantomStruct<char, f32> = PhantomStruct {
        first: 'Q',
        phantom: PhantomData,
    };

    // let struct2: PhantomStruct<char, f64> = PhantomStruct {
    //     first: 'Q',
    //     phantom: PhantomData,
    // };

    println!("tuple1 == tuple2 yields: {}", tuple1 == tuple2);
    println!("struct1 == struct2 yields: {}", struct1 == struct2);
}

#[derive(PartialEq)]
struct PhantomTuple<A, B>(A, PhantomData<B>);

#[derive(PartialEq)]
struct PhantomStruct<A, B> {
    first: A,
    phantom: PhantomData<B>,
}
